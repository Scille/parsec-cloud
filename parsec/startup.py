import logging
import argparse
from multiprocessing import Process
from setproctitle import getproctitle, setproctitle

from parsec.config import load_config, ConfigError


def _mk_bootstrap(server_name, start):

    def bootstrap():
        basename = getproctitle()
        print('Boot', basename)
        setproctitle(basename + ' -s ' + server_name)
        print('changed to', getproctitle())
        start()

    return bootstrap


def boot_servers(servers_factory, main_factory=None):
    processes = []
    for key, factory in servers_factory.items():
        if factory is main_factory:
            continue
        # p = Process(target=factory().start)
        p = Process(target=_mk_bootstrap(key, factory().start))
        p.start()
        processes.append(p)

    try:
        if main_factory:
            main_factory().start()
        else:
            for p in processes:
                p.join()
    except KeyboardInterrupt:
        print('Trying to gracefully exiting processes... ')
        for p in processes:
            p.terminate()
        for p in processes:
            p.join()
        print('Bye ;-)')


def execute_from_command_line():
    parser = argparse.ArgumentParser()
    parser.add_argument('--config', '-c', type=argparse.FileType('r'), default='parsec.yml',
                        help='Config file to use (default: parsec.yml)')
    parser.add_argument('--log-level', '-l', dest='loglevel', default='WARNING',
                        help='Log level (default: WARNING)',
                        choices=('DEBUG', 'INFO', 'WARNING', 'ERROR'))
    parser.add_argument('--servers', '-s', nargs='*', help='Only start given servers')
    parser.add_argument('--verbose', '-V', action='store_true', help="Improve output verbosity")
    parser.add_argument('--main', '-m',
                        help='Server to start as main process (useful for debugging)')
    args = parser.parse_args()
    logging.basicConfig(level=getattr(logging, args.loglevel))
    try:
        topology = load_config(args.config, verbose=args.verbose)
    except ConfigError as exc:
        raise SystemExit("Configuration error:\n" + exc.dump())

    if args.servers:
        servers_factory = {k: f for k, f in topology.servers_factory.items() if k in args.servers}
    else:
        servers_factory = topology.servers_factory
    if args.main:
        try:
            main_factory = servers_factory.pop(args.main)
        except KeyError:
            raise SystemError('Unknown server `%s`' % args.main)
    else:
        main_factory = None
    boot_servers(servers_factory, main_factory=main_factory)
