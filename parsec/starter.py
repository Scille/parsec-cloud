from itertools import namedtuple
import yaml


Component = namedtuple('Component', ('name', ))

def parse_component(component):
    name = component.get('name')
    if not name:
        raise ConfigError('`name` is mandatory')
    return Component(component)


def bootstrap(config_path):
    config = yaml.load(config_path)
    components = config.get('components', ())
    for c in components:
        c.get('name')

