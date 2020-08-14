'p4a example service using oscpy to communicate with main application.'
from random import sample, randint
from string import ascii_letters
from time import localtime, asctime, sleep

from oscpy.server import OSCThreadServer
from oscpy.client import OSCClient

CLIENT = OSCClient('localhost', 3002)

stopFlag = False


def ping(*_):
    'answer to ping messages'
    msg = 'service: ' + ''.join(sample(ascii_letters, randint(10, 20))) # <---------- 
    CLIENT.send_message(
        b'/message',
        [
            msg.encode('utf8'),
        ],
    )


def send_date():
    'send date to the application'
    global stopFlag

    if stopFlag:
        msg = 'service stopped'
    else:
        msg = asctime(localtime())

    CLIENT.send_message(
        b'/date',
        [msg.encode('utf8'), ]
    )

def stop():
    global stopFlag
    stopFlag=True
    SERVER.close()
    
if __name__ == '__main__':
    SERVER = OSCThreadServer()
    SERVER.listen('localhost', port=3000, default=True)
    SERVER.bind(b'/ping', ping)
    SERVER.bind(b'/stop', stop)    # stop service when receiving 'stop msg

    while not stopFlag:
        sleep(1)
        send_date()
