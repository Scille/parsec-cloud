__version__ = '1.0.0'
__author__ = 'Emmanuel Leblond'
__email__ = 'emmanuel.leblond@gmail.com'


from .base import ChainedIntent, Effect, do, TypeDispatcher, ComposedDispatcher, raise_
from .sync import sync_perform, base_sync_dispatcher
from .asyncio import asyncio_perform, base_asyncio_dispatcher
from .intents import Delay, Constant, Error, Func, base_dispatcher
