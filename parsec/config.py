import os
import inspect
import yaml
import traceback
from uuid import uuid4
from importlib import import_module


TOP_LEVEL_FIELDS = {'servers', 'services', 'clients'}


class ConfigError(Exception):
    def __init__(self, errors):
        self.errors = errors

    def dump(self):

        def _recursive_dump(error_dict, prefix=''):
            errors = []
            for key, error in error_dict.items():
                error_path = '%s/%s' % (prefix, key)
                if isinstance(error, dict):
                    errors += _recursive_dump(error, prefix=error_path)
                else:
                    errors.append('[%s] %s' % (error_path, error))
            return errors

        return '\n'.join(_recursive_dump(self.errors))


class Topology:
    def __init__(self):
        self.services_factory = {}
        self.servers_factory = {}
        self.clients_factory = {}
        self._objs = {}

    def add_service(self, id, service):
        self._add_obj(id, service)
        self.services_factory[id] = lambda: service.resolve(self)

    def add_client(self, id, client):
        self._add_obj(id, client)
        self.clients_factory[id] = lambda: client.resolve(self)

    def add_server(self, id, server):
        self._add_obj(id, server)
        self.servers_factory[id] = lambda: server.resolve(self)

    def _add_obj(self, id, obj):
        if id in self._objs:
            raise ConfigError('Object `%s` already defined.' % id)
        self._objs[id] = obj

    def get_obj(self, id):
        obj = self._objs.get(id, None)
        if not obj:
            raise ConfigError('Object `%s` not defined.' % id)
        elif isinstance(obj, LazyObj):
            obj = obj.resolve(self)
            self._objs[id] = obj
        return obj


class BaseLazy:
    def resolve(self, topology):
        raise NotImplementedError()

    def get_deps(self):
        raise NotImplementedError()


class LazyRef(BaseLazy):
    def __init__(self, id, field=None, expected_type=None):
        self.id = id
        self.expected_type = expected_type
        self.field = field

    def resolve(self, topology):
        obj = topology.get_obj(self.id)
        if self.expected_type and not isinstance(obj, self.expected_type):
            raise ConfigError('Field `%s` should resolve to type `%s`' %
                (self.field, self.expected_type))
        return obj

    def get_deps(self):
        return {self.id}


class LazyObj(BaseLazy):
    def __init__(self, id, cls, **params):
        self.id = id
        self._cls = cls
        self._params = params
        self._resolved_obj = None

    def resolve(self, topology):
        if not self._resolved_obj:
            resolved_params = {}
            for k, v in self._params.items():
                if isinstance(v, BaseLazy):
                    resolved_params[k] = v.resolve(topology)
                else:
                    resolved_params[k] = v
            self._resolved_obj = self._cls(**resolved_params)
        return self._resolved_obj

    def get_deps(self):
        deps = set()
        for k, v in self._params.items():
            if isinstance(v, BaseLazy):
                deps |= v.get_deps()
        return deps


class TopologyLoader:

    def __init__(self, verbose=False):
        self.verbose = verbose

    def _retrieve_by_import(self, path):
        path = path if path.startswith('parsec.') else 'parsec.' + path
        module_path, name = path.rsplit('.', 1)
        try:
            module = import_module(module_path)
            return getattr(module, name)
        except (ImportError, AttributeError):
            msg = 'Unknown ressource `%s`.' % path
            if self.verbose:
                msg += '\nError:\n' + traceback.format_exc()
            raise ConfigError(msg)

    def _check_fields(self, data, required=None, optional=None):
        errors = []
        data_fields = set(data.keys())
        required_fields = set(required or ())
        allowed_fields = required_fields | set(optional or ())
        unknown_fields = data_fields - allowed_fields
        if unknown_fields:
            errors.append('Unknown fields: %s.' % unknown_fields)
        missing_fields = required_fields - data_fields
        if missing_fields:
            errors.append('Missing fields: %s.' % missing_fields)
        type_checks = {}
        if isinstance(required, dict):
            type_checks.update(required)
        if isinstance(optional, dict):
            type_checks.update(optional)
        for k, t in type_checks.items():
            if k in data and not isinstance(data[k], t):
                errors.append('Field `%s` should be of type `%s`' % (k, t))
        if errors:
            raise ConfigError(errors)

    def _build_obj_from_config(self, data):
        """
        Config example::

            {
                'id': 'vfs-server-01',
                'class': 'vfs.VFSServer',
                'params': {'mount_point': '/tmp/foo'}
            }

        """
        self._check_fields(data, required={'class': str}, optional={'id': str, 'params': dict})
        cls = self._retrieve_by_import(data['class'])
        raw_params = data.get('params', {})
        params = self._cook_params_from_annotations(cls.__init__, raw_params)
        return LazyObj(data.get('id', uuid4().hex), cls, **params)

    def _cook_params_from_annotations(self, builder, params):
        argspec = inspect.getfullargspec(builder)
        anns = argspec.annotations
        unknown_fields = set(params.keys()) - set(anns.keys())
        if unknown_fields:
            raise ConfigError('Unknown fields: %s.' % unknown_fields)
        with_default_fields = set()
        if argspec.kwonlydefaults:
            with_default_fields |= set(argspec.kwonlydefaults)
        if argspec.args and argspec.defaults:
            with_default_fields |= set(k for k, _ in zip(reversed(argspec.args), argspec.defaults))
        missing_fields = set(anns.keys()) - set(params.keys()) - {'return'} - with_default_fields
        if missing_fields:
            raise ConfigError('Missing fields: %s.' % missing_fields)
        cooked = {}
        errors = []
        for k, v in params.items():
            t = anns[k]
            if t in (int, float, str, bytes, list, dict):
                if isinstance(v, str) and v.startswith('$'):
                    # Handle env var here
                    # If env var is not defined, use default value if available
                    env_var = v[1:]
                    if env_var in os.environ or k not in with_default_fields:
                        value = os.environ.get(v[1:], '')
                        try:
                            if t == bytes:
                                cooked[k] = value.encode()
                            else:
                                cooked[k] = t(value)
                        except TypeError:
                            errors.append('Field `%s` should be of type `%s`' % (k, t))
                elif isinstance(v, t):
                    # Default json/yaml types
                    cooked[k] = v
                else:
                    errors.append('Field `%s` should be of type `%s`' % (k, t))
            else:
                try:
                    # Otherwise consider the value is a python path to a parsec resource
                    if isinstance(v, dict):
                        # Build object from the given dict
                        cooked[k] = self._build_obj_from_config(v)
                    elif isinstance(v, str):
                        # Retrieve object
                        cooked[k] = LazyRef(v, field=k, expected_type=t)
                    else:
                        errors.append('Field `%s` should resolve to type `%s`' % (k, t))
                except ConfigError as exc:
                    errors.append(exc.errors)
        if errors:
            raise ConfigError(errors)
        return cooked

    def _parse_list_objs(self, config_list):
        objs = []
        errors = {}
        for i, obj_config in enumerate(config_list):
            try:
                objs.append(self._build_obj_from_config(obj_config))
            except ConfigError as exc:
                errors[i] = exc.errors
        if errors:
            raise ConfigError(errors)
        return objs

    def load(self, raw_yaml):
        self._check_fields(raw_yaml, optional={k: list for k in TOP_LEVEL_FIELDS})
        topology = Topology()
        errors = {}
        try:
            for service in self._parse_list_objs(raw_yaml.get('services')):
                topology.add_service(service.id, service)
        except ConfigError as exc:
            errors['services'] = exc.errors
        try:
            for client in self._parse_list_objs(raw_yaml.get('clients')):
                topology.add_client(client.id, client)
        except ConfigError as exc:
            errors['clients'] = exc.errors
        try:
            for server in self._parse_list_objs(raw_yaml.get('servers')):
                topology.add_server(server.id, server)
        except ConfigError as exc:
            errors['servers'] = exc.errors
        if errors:
            raise ConfigError(errors)
        return topology


def load_config(fp, verbose=False):
    return loads_config(fp.read(), verbose=verbose)


def loads_config(raw_data, verbose=False):
    raw_yaml = yaml.safe_load(raw_data)
    loader = TopologyLoader(verbose=verbose)
    return loader.load(raw_yaml)
