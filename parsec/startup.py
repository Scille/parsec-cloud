import logging
import argparse
from multiprocessing import Process

from parsec.config import load_config, ConfigError


def boot_servers(servers_factory, main_factory=None):
    processes = []
    for factory in servers_factory:
        if factory is main_factory:
            continue
        p = Process(target=factory().start)
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
        servers_factory = [f for k, f in topology.servers_factory.items() if k in args.servers]
    else:
        servers_factory = list(topology.servers_factory.values())
    if args.main:
        try:
            main_factory = topology.servers_factory[args.main]
        except KeyError:
            raise SystemError('Unknown server `%s`' % args.main)
        servers_factory.remove(main_factory)
    else:
        main_factory = None
    boot_servers(servers_factory, main_factory=main_factory)
