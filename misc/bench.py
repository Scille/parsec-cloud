#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# -*- coding: utf-8 -*-

############################# Naive-Bench ###################################
# https://github.com/bkryza/naive-bench
#
# The MIT License (MIT)
# Copyright (c) 2016 Bartosz Kryza <bkryza at gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
# DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
# OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
# OR OTHER DEALINGS IN THE SOFTWARE.
#
#from __future__ import print_function

import random, time, optparse
import socket, sys, os, re, math

from os import system
from multiprocessing import Process, Manager


############################# Humanize ###################################
# https://github.com/jmoiron/humanize
#
# Copyright (c) 2010 Jason Moiron and Contributors
# 
# Permission is hereby granted, free of charge, to any person obtaining
# a copy of this software and associated documentation files (the
# "Software"), to deal in the Software without restriction, including
# without limitation the rights to use, copy, modify, merge, publish,
# distribute, sublicense, and/or sell copies of the Software, and to
# permit persons to whom the Software is furnished to do so, subject to
# the following conditions:
# 
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
# NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
# LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
# OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
# WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

"""Bits & Bytes related humanization."""

suffixes = {
    'decimal': ('kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'),
    'binary': ('KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'),
    'gnu': "KMGTPEZY",
}


def naturalsize(value, binary=False, gnu=False, format='%.1f'):
    """Format a number of byteslike a human readable filesize (eg. 10 kB).  By
    default, decimal suffixes (kB, MB) are used.  Passing binary=true will use
    binary suffixes (KiB, MiB) are used and the base will be 2**10 instead of
    10**3.  If ``gnu`` is True, the binary argument is ignored and GNU-style
    (ls -sh style) prefixes are used (K, M) with the 2**10 definition.
    Non-gnu modes are compatible with jinja2's ``filesizeformat`` filter."""
    if gnu: suffix = suffixes['gnu']
    elif binary: suffix = suffixes['binary']
    else: suffix = suffixes['decimal']

    base = 1024 if (gnu or binary) else 1000
    bytes = float(value)

    if bytes == 1 and not gnu: return '1 Byte'
    elif bytes < base and not gnu: return '%d Bytes' % bytes
    elif bytes < base and gnu: return '%dB' % bytes

    for i,s in enumerate(suffix):
        unit = base ** (i+2)
        if bytes < unit and not gnu:
            return (format + ' %s') % ((base * bytes / unit), s)
        elif bytes < unit and gnu:
            return (format + '%s') % ((base * bytes / unit), s)
    if gnu:
        return (format + '%s') % ((base * bytes / unit), s)
    return (format + ' %s') % ((base * bytes / unit), s)

##########################################################################


#
# Global constants
#
kibybytes = ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB']
kilobytes = ['KB', 'MB', 'GB', 'TB', 'PB', 'EB']

process_manager = Manager()


#
# Initialize CSV column labels
#
storage_name_label = "STORAGE NAME"
number_files_label = "FILE COUNT"
average_file_size_label = "AVERAGE FILE SIZE [b]"
create_files_label = "CREATE TIME [s]"
create_files_size_label = "CREATE SIZE [b]"
overwrite_files_label = "WRITE TIME [s]"
overwrite_files_size_label = "WRITE SIZE [b]"
linear_read_label = "LINEAR READ TIME [s]"
linear_read_size_label = "LINEAR READ SIZE [b]"
random_read_label = "RANDOM READ TIME [s]"
random_read_size_label = "RANDOM READ SIZE [b]"
delete_label = "DELETE"

__test_data_dir = "naive-bench-data"



def get_random_file_size(filesize, dev):
    """
    Get randomized file size based on average 'filesize' and deviation range.
    """
    
    min_range = (1.0-dev)*filesize
    max_range = (1.0+dev)*filesize
    
    return int( (max_range-min_range)*random.random() + min_range )


def get_random_data(size):
    """
    Create a an array of specified size filled with a random bytes
    """
    return bytearray(os.urandom(size))


def parse_file_size(file_size_string):
    """
    This function parses the file sizes supporting both conventions 
    (i.e. KiB and KB)
    """

    file_size = float('nan')
    #
    # First check if the number is in bytes without suffix
    # if it's not try to match a known suffix
    #
    try:
        file_size = int(file_size_string)
        return file_size
    except ValueError:
        parse_result = re.split(r'([\.\d]+)', file_size_string)
        try:
            file_size = float(parse_result[1])
            file_size_suffix = -1
            if parse_result[2] in kibybytes:
                file_size *= math.pow(1024, (kibybytes.index(parse_result[2])+1))
                return file_size
            elif parse_result[2] in kilobytes:
                file_size *= math.pow(1000, (kilobytes.index(parse_result[2])+1))
                return file_size
            else:
                return float('nan')
        except ValueError:
            return float('nan')
        return False


def init_child_process(write_lock):
    """
    """

    pass


def drop_caches():
    """
    Drops file cache
    """

    cmd = ""
    if sys.platform == "linux" or sys.platform == "linux2":
        cmd = "sudo sh -c 'sync ; echo 3 > /proc/sys/vm/drop_caches'"
    elif sys.platform == "darwin":
        cmd = "sudo sh -c 'sync; purge'"
    else:
        print ( sys.platform, " platform is not supported - exiting." )
        sys.exit(1)

    system(cmd)


def format_progress_message(name, progress, total, suffix, width=40, \
                            numtype='numeric'):
    """
    Formats the progress
    """
    if numtype == 'normal':
        p = int((progress*width)/total)
        percentage = int((progress*100)/total)
        return name + (" [%-40s] %d%%" % ('='*p, percentage)) + " " \
            + progress  + "/"\
            + total  \
            + " | " \
            + suffix + "        "
    elif numtype == 'filesize':
        p = int((progress*width)/total)
        percentage = int((progress*100)/total)
        return name + (" [%-40s] %d%%" % ('='*p, percentage)) + ", " \
+ naturalsize(progress)  + " of "\
            + naturalsize(total)  \
            + " | " \
            + suffix + "        "


def run_benchmark(benchmark, \
                  filecount, threadcount, deviation, blocksize, \
                  threads_results, threads_progress_messages):
    """
    This is a generic function for running naive benchmarks
    """

    #
    # Initialize barrier lock to wait until the threads initialize before
    # starting time measurement
    #
    start_barrier = Manager().Barrier(threadcount+1)

    #
    # Prepapre a list of arguments for each benchmark task
    #
    benchmark_args = []
    for tidx in range(threadcount):

        r = None
        if(filecount == threadcount):
            r = [tidx]
        else:
            low_range = int(tidx*(filecount/threadcount))
            high_range = int((tidx+1)*(filecount/threadcount))
            r = list(range(low_range, high_range))

        benchmark_args.append(\
            (tidx, r, filesize, deviation, blocksize, __test_data_dir, \
               threads_results, threads_progress_messages, start_barrier))
        threads_results[tidx] = 0
        threads_progress_messages[tidx] = "Starting task "+str(tidx)

    #
    # Create the process pool and run the benchmark
    #
    progress_bars = []
    for i in range(threadcount):
        child = Process(target=benchmark, \
                        args=benchmark_args[i])
        child.start()
        threads.append(child)

    #
    # Wait for all benchmark tasks to initialize
    #
    start_barrier.wait()

    start_time = time.time()
    #
    # Wait for the threads to complete and the progress every 
    # 0.5 second
    #
    time.sleep(0.5)
    while any(thread.is_alive() for thread in threads):
        time.sleep(0.5)
        for i in range(threadcount):
            print(threads_progress_messages[i], file=sys.stderr)
        for i in range(threadcount):
            sys.stderr.write("\x1b[A")

    for i in range(threadcount):
        print(threads_progress_messages[i], file=sys.stderr)

    real_execution_time = time.time() - start_time

    return real_execution_time


def file_create_benchmark(task_id, file_ids, filesize, deviation, \
                          blocksize, test_data_dir, \
                          thread_results, thread_progress_messages, \
                          start_barrier):
    """
    Task which creates a set of test files and measures total time
    """

    total_written_bytes = 0

    #
    # Generate random file sizes and calculate total size for this task
    #
    random_file_sizes = \
                  [get_random_file_size(filesize, deviation) for i in file_ids]
    total_size_to_write = sum(random_file_sizes)

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                "???",
                                width=40, numtype='filesize')

    randdata = get_random_data(blocksize)

    start_barrier.wait()
    start_time = time.time()
    for i in range(len(file_ids)):
        #
        # Create random size file
        #
        rand_size = random_file_sizes[i]
        outfile = open(test_data_dir + "/" + str(file_ids[i]), "wb")
        #
        # Rewrite random device to the output file in 'blocksize' blocks
        #
        file_written_bytes = 0
        while(file_written_bytes + blocksize < rand_size):
            block_written_bytes = outfile.write(randdata)
            file_written_bytes += block_written_bytes
            total_written_bytes += block_written_bytes
            #
            # Format progress message
            #
            create_current_throughput = \
                naturalsize(total_written_bytes/(time.time()-start_time)) \
                + "/s"

            thread_progress_messages[task_id] = \
                format_progress_message("Task #" + str(task_id),
                                        total_written_bytes,
                                        total_size_to_write,
                                        create_current_throughput,
                                        width=40, numtype='filesize')

        #
        # Write remainder of the file
        #
        block_written_bytes = \
                    outfile.write(randdata[0:rand_size - file_written_bytes])
        total_written_bytes += block_written_bytes

        #
        # Truncate if configured for consecutive write benchmarks
        #
        if options.truncate:
            outfile.truncate(0)

    end_time = time.time() - start_time

    current_throughput = \
        naturalsize(total_written_bytes/(time.time()-start_time)) \
        + "/s"

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                current_throughput,
                                width=40, numtype='filesize')

    thread_results[task_id] = (total_written_bytes, end_time)



def file_write_benchmark(task_id, file_ids, filesize, deviation, \
                          blocksize, test_data_dir, \
                          thread_results, thread_progress_messages, \
                          start_barrier):
    """
    Benchmark testing writing to existing files
    """

    total_written_bytes = 0

    #
    # Generate random file sizes and calculate total size for this task
    # \todo fix when files have random sizes
    random_file_sizes = \
                  [get_random_file_size(filesize, deviation) for i in file_ids]
    total_size_to_write = sum(random_file_sizes)

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                "???",
                                width=40, numtype='filesize')


    randdata = get_random_data(blocksize)

    start_barrier.wait()
    start_time = time.time()
    for i in range(len(file_ids)):
        #
        # Create random size file
        #
        rand_size = random_file_sizes[i]
        outfile = open(test_data_dir + "/" + str(file_ids[i]), "wb")
        #
        # Rewrite random device to the output file in 'blocksize' blocks
        #
        file_written_bytes = 0
        while(file_written_bytes + blocksize < rand_size):
            block_written_bytes = outfile.write(randdata)
            file_written_bytes += block_written_bytes
            total_written_bytes += block_written_bytes
            #
            # Format progress message
            #
            create_current_throughput = \
                naturalsize(total_written_bytes/(time.time()-start_time)) \
                + "/s"

            thread_progress_messages[task_id] = \
                format_progress_message("Task #" + str(task_id),
                                        total_written_bytes,
                                        total_size_to_write,
                                        create_current_throughput,
                                        width=40, numtype='filesize')

        #
        # Write remainder of the file
        #
        block_written_bytes = \
                    outfile.write(randdata[0:rand_size - file_written_bytes])
        total_written_bytes += block_written_bytes

    end_time = time.time() - start_time

    current_throughput = \
        naturalsize(total_written_bytes/(time.time()-start_time)) \
        + "/s"

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                current_throughput,
                                width=40, numtype='filesize')

    thread_results[task_id] = (total_written_bytes, end_time)


def file_random_write_benchmark(task_id, file_ids, filesize, deviation, \
                          blocksize, test_data_dir, \
                          thread_results, thread_progress_messages, \
                          start_barrier):
    """
    Benchmark testing writing to existing files
    """

    total_written_bytes = 0

    #
    # Generate random file sizes and calculate total size for this task
    # \todo fix when files have random sizes
    random_file_sizes = \
                  [get_random_file_size(filesize, deviation) for i in file_ids]
    total_size_to_write = sum(random_file_sizes)

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                "???",
                                width=40, numtype='filesize')


    randdata = get_random_data(blocksize)

    start_barrier.wait()
    start_time = time.time()
    for i in range(len(file_ids)):
        #
        # Create random size file
        #
        rand_size = random_file_sizes[i]
        outfile = open(test_data_dir + "/" + str(file_ids[i]), "wb")
        #
        # Prepare a shuffled list of block indexes to write in random order
        #
        random_block_indexes = [i for i in range(0,int(rand_size/blocksize))]
        random.shuffle(random_block_indexes)
        
        file_written_bytes = 0
        for block_index in random_block_indexes:
            outfile.seek(block_index*blocksize, 0)
            block_written_bytes = outfile.write(randdata)
            file_written_bytes += block_written_bytes
            total_written_bytes += block_written_bytes

            #
            # Format progress message
            #
            create_current_throughput = \
                naturalsize(total_written_bytes/(time.time()-start_time)) \
                + "/s"

            thread_progress_messages[task_id] = \
                format_progress_message("Task #" + str(task_id),
                                        total_written_bytes,
                                        total_size_to_write,
                                        create_current_throughput,
                                        width=40, numtype='filesize')

        #
        # Write remainder of the file
        #
        block_written_bytes = \
                    outfile.write(randdata[0:rand_size - file_written_bytes])
        total_written_bytes += block_written_bytes

    end_time = time.time() - start_time

    current_throughput = \
        naturalsize(total_written_bytes/(time.time()-start_time)) \
        + "/s"

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_written_bytes,
                                total_size_to_write,
                                current_throughput,
                                width=40, numtype='filesize')

    thread_results[task_id] = (total_written_bytes, end_time)


def file_linear_read_benchmark(task_id, file_ids, filesize, deviation, \
                               blocksize, test_data_dir, \
                               thread_results, thread_progress_messages, \
                               start_barrier):
    """
    Benchmark testing the time of linear reading from files 
    """

    total_read_bytes = 0

    #
    # Calculate the size of files to read
    #
    file_sizes = {}
    for f in file_ids:
        file_sizes[f] = os.path.getsize(test_data_dir+"/"+str(f))

    total_size_to_read = sum(file_sizes.values())

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_read_bytes,
                                total_size_to_read,
                                "???",
                                width=40, numtype='filesize')


    outfile = open("/dev/null", "wb")
    start_barrier.wait()
    start_time = time.time()

    for i in range(len(file_ids)):
        #
        # Open file
        #
        infile = open(test_data_dir + "/" + str(file_ids[i]), "rb")

        #
        # Read the file in blocks
        #
        file_read_bytes = 0
        
        while(file_read_bytes + blocksize < file_sizes[file_ids[i]]):
            block_read_bytes = outfile.write(infile.read(blocksize))
            file_read_bytes += block_read_bytes
            total_read_bytes += block_read_bytes
            #
            # Format progress message
            #
            current_throughput = \
                naturalsize(total_read_bytes/(time.time()-start_time)) \
                + "/s"

            thread_progress_messages[task_id] = \
                format_progress_message("Task #" + str(task_id),
                                        total_read_bytes,
                                        total_size_to_read,
                                        current_throughput,
                                        width=40, numtype='filesize')

        #
        # Write remainder of the file
        #
        block_read_bytes = \
            outfile.write(infile.read(file_sizes[file_ids[i]]-file_read_bytes))
        total_read_bytes += block_read_bytes

    outfile.close()
    end_time = time.time() - start_time
    current_throughput = \
        naturalsize(total_read_bytes/(time.time()-start_time)) \
        + "/s"

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_read_bytes,
                                total_size_to_read,
                                current_throughput,
                                width=40, numtype='filesize')
    thread_results[task_id] = (total_read_bytes, end_time)


def file_random_read_benchmark(task_id, file_ids, filesize, deviation, \
                               blocksize, test_data_dir, \
                               thread_results, thread_progress_messages, \
                               start_barrier):
    """
    Benchmark measures the time of random read from files using seek
    """

    total_read_bytes = 0

    #
    # Calculate the size of files to read
    #
    file_sizes = {}
    for f in file_ids:
        file_sizes[f] = os.path.getsize(test_data_dir+"/"+str(f))

    total_size_to_read = sum(file_sizes.values())

    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_read_bytes,
                                total_size_to_read,
                                "???",
                                width=40, numtype='filesize')


    outfile = open("/dev/null", "wb")
    start_barrier.wait()
    start_time = time.time()

    for i in range(len(file_ids)):
        #
        # Open file
        #
        infile = open(test_data_dir + "/" + str(file_ids[i]), "rb")
        infile_size = file_sizes[file_ids[i]]

        #
        # Read the file in blocks
        #
        file_read_bytes = 0
        
        #
        # Prepare a shuffled list of block indexes to read in random order
        #
        random_block_indexes = [i for i in range(0,int(infile_size/blocksize))]
        random.shuffle(random_block_indexes)
        
        for block_index in random_block_indexes:
            infile.seek(block_index*blocksize, 0)
            block_read_bytes = outfile.write(infile.read(blocksize))
            file_read_bytes += block_read_bytes
            total_read_bytes += block_read_bytes
            #
            # Format progress message
            #
            current_throughput = \
                naturalsize(total_read_bytes/(time.time()-start_time)) \
                + "/s"

            thread_progress_messages[task_id] = \
                format_progress_message("Task #" + str(task_id),
                                        total_read_bytes,
                                        total_size_to_read,
                                        current_throughput,
                                        width=40, numtype='filesize')

        #
        # Write remainder of the file
        #
        block_read_bytes = \
            outfile.write(infile.read(file_sizes[file_ids[i]]-file_read_bytes))
        total_read_bytes += block_read_bytes

    outfile.close()
    end_time = time.time() - start_time
    current_throughput = \
        naturalsize(total_read_bytes/(time.time()-start_time)) \
        + "/s"
    thread_progress_messages[task_id] = \
        format_progress_message("Task #" + str(task_id),
                                total_read_bytes,
                                total_size_to_read,
                                current_throughput,
                                width=40, numtype='filesize')
    thread_results[task_id] = (total_read_bytes, end_time)


if __name__ == '__main__':
    #
    # Parse command line options
    #
    parser = optparse.OptionParser()

    parser.add_option('-f', '--filecount',
        action="store", dest="filecount", type='int',
        help="Number of files to create", default=100)

    parser.add_option('-s', '--filesize', type='string',
        action="store", dest="filesize",
        help="""Average created file size. The file sizes will be random
in the range (0.5*filesize, 1.5*filesize].""",
        default=1024*1024)

    parser.add_option('-b', '--blocksize', type='string',
        action="store", dest="blocksize",
        help="""Size of data block for random read test.""", default=1024)

    parser.add_option('-n', '--name',
        action="store", dest="name",
        help="""Name of storage which identifies the performed test.
Defaults to hostname.""",
        default=socket.gethostname())

    parser.add_option('-c', '--csv',
        action="store_true", dest="csv",
        help="Generate CSV output.", default=False)

    parser.add_option('-H', '--no-header',
        action="store_true", dest="skipheader",
        help="Skip CSV header.", default=False)

    parser.add_option('-r', '--read-only',
        action="store_true", dest="readonly",
        help="""This test will only perform read tests.
It assumes that the current folder contains 'naive-bench-data' folder
with test files uniformly numbered in the specified range.""",
        default=False)

    parser.add_option('-w', '--write-only',
        action="store_true", dest="writeonly",
        help="""This test will only perform write tests.
This option can be used to create data on storage for peforming
remote read tests.""", default=False)

    parser.add_option('-k', '--keep',
        action="store_true", dest="keep",
        help="""Keep the files after running the test.""", default=False)

    parser.add_option('-u', '--truncate-after-create',
        action="store_true", dest="truncate",
        help="""Truncates the created files to 0 size so that
consecutive write benchmarks work with empty files.""", default=False)

    parser.add_option('-F', '--force',
        action="store_true", dest="force",
        help="""Run the test even when the available storage
size is too small.""",
        default=False)

    parser.add_option('-d', '--deviation', type='float',
        action="store", dest="deviation",
        help="""Generate the files with random size in range
((1.0-deviation)*filesize, (1.0+deviation)*filesize].""",
        default=0.0)

    parser.add_option('-t', '--thread-count', type='int',
        action="store", dest="threadcount",
        help="""Number of threads to execute for each test.""",
        default=4)

    parser.add_option('-P', '--no-purge',
        action="store_true", dest="nopurge",
        help="""If specified, disables cache clearing between steps.""",
        default=False)

    #
    # Parse the command line
    #
    options, args = parser.parse_args()

    filesize = parse_file_size(options.filesize)
    filecount = int(options.filecount)
    blocksize = parse_file_size(options.blocksize)
    deviation = options.deviation
    threadcount = options.threadcount
    dropcaches = not options.nopurge

    if math.isnan(filesize):
        print("Invalid filesize - exiting.", file=sys.stderr)
        sys.exit(2)
    else:
        filesize = int(filesize)

    if math.isnan(blocksize):
        print("Invalid blocksize - exiting.", file=sys.stderr)
        sys.exit(2)
    else:
        blocksize = int(blocksize)

    if blocksize > filesize:
        print("Blocksize must not be larger than filesize - exiting.", \
              file=sys.stderr)
        sys.exit(2)

    if deviation < 0.0 or deviation > 0.9:
        print("Deviation must be in range [0.0, 0.9] - exiting.", \
              file=sys.stderr)
        sys.exit(2)

    if threadcount>filecount or filecount%threadcount != 0:
        print("Total file count must be a multiple of thread count - exiting.", \
              file=sys.stderr)
        sys.exit(2)

    #
    # Calculate available disk space on the current volume
    #
    st = os.statvfs(os.getcwd())
    available_disk_space = st.f_bavail * st.f_frsize

    #
    # Printout basic benchmark parameters
    #
    print("------------------------------", file=sys.stderr)
    print('Starting test', file=sys.stderr)
    print("  Number of files: ", filecount, file=sys.stderr)
    print('  Average file size: ', naturalsize(filesize), \
          file=sys.stderr)
    print('  Maximum disk space needed: ', \
          naturalsize(filesize * filecount * (1.0+deviation)),
          file=sys.stderr)
    print('  Available disk space: ', \
          naturalsize(available_disk_space), file=sys.stderr)
    print('  Number of parallel threads:', \
          threadcount, file=sys.stderr)
    print('------------------------------', file=sys.stderr)


    #
    # Check available disk space for test
    #
    if (filesize * filecount * (1.0+deviation)) > available_disk_space \
                                                 and not options.force:
        print("Not enough disk space to perform test - exiting.", \
              file=sys.stderr)
        sys.exit(1)

    #
    # Check conflicting options
    #
    if options.readonly and options.writeonly:
        print("Cannot perform readonly and writeonly test - exiting.",
              file=sys.stderr)
        sys.exit(2)

    if options.filecount < 1:
        print("Cannot perform test with no files - exiting.", file=sys.stderr)
        sys.exit(2)

    #
    # Initialize time variables
    #
    create_files_time = float('NaN')
    create_files_bytes_size = 0
    overwrite_files_time = float('NaN')
    overwrite_files_bytes_size = 0
    linear_read_time = float('NaN')
    linear_read_bytes_size = 0
    random_read_time = float('NaN')
    random_read_bytes_size = 0
    delete_time = float('NaN')


    print("\n\nCreating test folder 'naive-bench-data'...", end="", \
           file=sys.stderr)
    #
    # Cleanup old test data
    #
    system("rm -rf naive-bench-data")
    starttime = time.time()
    system("mkdir naive-bench-data")
    endtime = time.time() - starttime
    print("DONE [%d s]\n"%(endtime), file=sys.stderr)

    if dropcaches:
        print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
        drop_caches()
        print(" DONE", file=sys.stderr)

    ##########
    #
    # Start file creation benchmark
    #
    #
    if not options.readonly:
        threads = []
        threads_results = process_manager.dict()
        threads_progress_messages = process_manager.dict()
        print("\n--- INITIALIZING FILE CREATION BENCHMARK...\n", file=sys.stderr)
        
        create_files_time = run_benchmark(file_create_benchmark, \
                                        filecount, threadcount, deviation, \
                                        blocksize, threads_results, \
                                        threads_progress_messages)

        #
        # Calculate total benchmark size and time
        #
        create_files_bytes_size = sum(s[0] for s in threads_results.values())

        print("", file=sys.stderr)
        print("--- CREATED " + str(filecount) + " FILES OF TOTAL SIZE " \
            + str(naturalsize(create_files_bytes_size)) + " IN " \
            + str(create_files_time) + "s", file=sys.stderr)
        print("--- THROUGHPUT: " \
            + str(naturalsize(create_files_bytes_size/create_files_time))\
            + "/s", file=sys.stderr)
        print("", file=sys.stderr)

        if dropcaches:
            print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
            drop_caches()
            print(" DONE", file=sys.stderr)

        ##########
        #
        # Start file random write benchmark
        #
        #
        threads = []
        threads_results = process_manager.dict()
        threads_progress_messages = process_manager.dict()
        print("\n--- INITIALIZING FILE RANDOM WRITE BENCHMARK...\n", file=sys.stderr)
        
        overwrite_files_time = run_benchmark(file_random_write_benchmark, \
                                            filecount, threadcount, deviation, \
                                            blocksize, threads_results, \
                                            threads_progress_messages)

        #
        # Calculate total benchmark size and time
        #
        overwrite_files_bytes_size = sum(s[0] for s in threads_results.values())

        print("", file=sys.stderr)
        print("--- WRITTE " + str(filecount) + " FILES WITH TOTAL SIZE" \
            + str(naturalsize(overwrite_files_bytes_size)) + " IN " \
            + str(overwrite_files_time) + "s", file=sys.stderr)
        print("--- THROUGHPUT: " \
            + str(naturalsize(\
                                overwrite_files_bytes_size/overwrite_files_time)) \
            + "/s", file=sys.stderr)
        print("", file=sys.stderr)
        
        if dropcaches:
            print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
            drop_caches()
            print(" DONE", file=sys.stderr)

        ##########
        #
        # Start file overwrite benchmark
        #
        #
        threads = []
        threads_results = process_manager.dict()
        threads_progress_messages = process_manager.dict()
        print("\n--- INITIALIZING FILE WRITE BENCHMARK...\n", file=sys.stderr)
        
        overwrite_files_time = run_benchmark(file_write_benchmark, \
                                            filecount, threadcount, deviation, \
                                            blocksize, threads_results, \
                                            threads_progress_messages)

        #
        # Calculate total benchmark size and time
        #
        overwrite_files_bytes_size = sum(s[0] for s in threads_results.values())

        print("", file=sys.stderr)
        print("--- OVERWRITTEN " + str(filecount) + " FILES WITH TOTAL SIZE" \
            + str(naturalsize(overwrite_files_bytes_size)) + " IN " \
            + str(overwrite_files_time) + "s", file=sys.stderr)
        print("--- THROUGHPUT: " \
            + str(naturalsize(\
                                overwrite_files_bytes_size/overwrite_files_time)) \
            + "/s", file=sys.stderr)
        print("", file=sys.stderr)
        
        if dropcaches:
            print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
            drop_caches()
            print(" DONE", file=sys.stderr)


    ##########
    #
    # Start linear read benchmark
    #
    #
    if not options.writeonly:
        threads = []
        threads_results = process_manager.dict()
        threads_progress_messages = process_manager.dict()
        print("\n--- INITIALIZING FILE LINEAR READ BENCHMARK...\n", file=sys.stderr)
        
        linear_read_time = run_benchmark(file_linear_read_benchmark, \
                                             filecount, threadcount, deviation, \
                                             blocksize, threads_results, \
                                             threads_progress_messages)

        #
        # Calculate total benchmark size and time
        #
        linear_read_bytes_size = sum(s[0] for s in threads_results.values())

        print("", file=sys.stderr)
        print("--- READ " + str(filecount) + " FILES WITH TOTAL SIZE " \
              + str(naturalsize(linear_read_bytes_size)) + " IN " \
              + str(linear_read_time) + "s", file=sys.stderr)
        print("--- THROUGHPUT: " \
              + str(naturalsize(linear_read_bytes_size/linear_read_time)) \
              + "/s", file=sys.stderr)
        print("", file=sys.stderr)
        
        if dropcaches:
            print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
            drop_caches()
            print(" DONE", file=sys.stderr)


    ##########
    #
    # Start random read benchmark
    #
    #
    if not options.writeonly:
        threads = []
        threads_results = process_manager.dict()
        threads_progress_messages = process_manager.dict()
        print("\n--- INITIALIZING FILE RANDOM READ BENCHMARK...\n", file=sys.stderr)
        
        random_read_time = run_benchmark(file_random_read_benchmark, \
                                             filecount, threadcount, deviation, \
                                             blocksize, threads_results, \
                                             threads_progress_messages)

        #
        # Calculate total benchmark size and time
        #
        random_read_bytes_size = sum(s[0] for s in threads_results.values())

        print("", file=sys.stderr)
        print("--- READ " + str(filecount) + " FILES WITH TOTAL SIZE " \
              + str(naturalsize(random_read_bytes_size)) + " IN " \
              + str(random_read_time) + "s", file=sys.stderr)
        print("--- THROUGHPUT: " \
              + str(naturalsize(random_read_bytes_size/random_read_time)) \
              + "/s", file=sys.stderr)
        print("", file=sys.stderr)

        if dropcaches:
            print("\n--- DROPPING FILE CACHE...", end="", file=sys.stderr)
            drop_caches()
            print(" DONE", file=sys.stderr)


    #
    # Delete the entire test folder
    #
    if not options.keep:
        print("\n--- CLEANING UP...", end="", file=sys.stderr)
        starttime = time.time()
        system("rm -rf naive-bench-data")
        delete_time = time.time() - starttime
        print("DONE [%d s]"%(delete_time), file=sys.stderr)

    print(file=sys.stderr)
    print(file=sys.stderr)

    #
    # Print CSV on stdout
    #
    if options.csv:
        if not options.skipheader:
            print(storage_name_label + ";" \
                  + number_files_label + ";" \
                  + average_file_size_label + ";" \
                  + create_files_label + ";" \
                  + create_files_size_label + ";" \
                  + overwrite_files_label + ";" \
                  + overwrite_files_size_label + ";"\
                  + linear_read_label + ";" \
                  + linear_read_size_label + ";"\
                  + random_read_label + ";" \
                  + random_read_size_label + ";"\
                  + delete_label)

        print(options.name + ";" \
              + str(filecount) + ';' \
              + str(filesize) + ';' \
              + str(create_files_time) + ';' \
              + str(create_files_bytes_size) + ';' \
              + str(overwrite_files_time) + ';' \
              + str(overwrite_files_bytes_size) + ';' \
              + str(linear_read_time) + ';' \
              + str(linear_read_bytes_size) + ';' \
              + str(random_read_time) + ';' \
              + str(random_read_bytes_size) + ';' \
              + str(delete_time))
