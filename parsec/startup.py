import signal
import logging
import argparse
from multiprocessing import Process

from parsec.config import load_config


def boot_servers(servers_factory):
    processes = []
    for factory in servers_factory:
        p = Process(target=factory().start)
        p.start()
        processes.append(p)

    def graceful_exit():
        for p in processes:
            p.terminate()

    signal.signal(signal.SIGINT, graceful_exit)
    signal.signal(signal.SIGTERM, graceful_exit)

    for p in processes:
        p.join()


def execute_from_command_line():
    parser = argparse.ArgumentParser()
    parser.add_argument('--config', '-c', type=argparse.FileType('r'), default='parsec.yml',
                        help='Config file to use (default: parsec.yml)')
    parser.add_argument('--log-level', '-l', dest='loglevel', default='WARNING',
                        help='Log level (default: WARNING)',
                        choices=('DEBUG', 'INFO', 'WARNING', 'ERROR'))
    parser.add_argument('--servers', '-s', nargs='*', help='Only start given servers')
    args = parser.parse_args()
    logging.basicConfig(level=getattr(logging, args.loglevel))
    topology = load_config(args.config)

    if args.servers:
        servers_factory = [f for k, f in topology.servers_factory.items() if k in args.servers]
    else:
        servers_factory = topology.servers_factory.values()
    boot_servers(servers_factory)
