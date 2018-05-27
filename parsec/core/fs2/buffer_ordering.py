import attr
import bisect
from math import inf


class OrderByStart:
    def __eq__(self, other):
        if not isinstance(other, OrderByStart):
            return NotImplemented
        return self.start == other.start

    def __lt__(self, other):
        if not isinstance(other, OrderByStart):
            return NotImplemented
        return self.start < other.start


@attr.s(slots=True)
class Buffer(OrderByStart):
    start = attr.ib()
    end = attr.ib()
    data = attr.ib()

    @property
    def size(self):
        return self.end - self.start


@attr.s(slots=True)
class ContiguousSpace(OrderByStart):
    start = attr.ib()
    end = attr.ib()
    buffers = attr.ib()


@attr.s(slots=True)
class UncontiguousSpace(OrderByStart):
    start = attr.ib()
    end = attr.ib()
    spaces = attr.ib()


@attr.s(slots=True)
class InBufferSpace(OrderByStart):
    start = attr.ib()
    end = attr.ib()
    buffer = attr.ib()

    buffer_slice_start = attr.ib(init=False)
    buffer_slice_end = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.buffer_slice_start = self.start - self.buffer.start
        self.buffer_slice_end = self.end - self.buffer.end

    @property
    def size(self):
        return self.end - self.start

    def slice_needed(self):
        return self.buffer_slice_start != 0 or self.buffer_slice_end != self.buffer.size


def is_overlaid(buff1, buff2):
    return (
        buff1.start <= buff2.start <= buff1.end or buff1.start <= buff2.end <= buff1.end or
        buff2.start < buff1.start < buff2.end or buff2.start < buff1.end <= buff2.end
    )


def _trim_buffers(buffers, new_start=None, new_end=None):
    trimmed = []

    for buff_space in buffers:
        buff_start = buff_space.start
        buff_end = buff_space.end

        if new_start is not None:
            if buff_end <= new_start:
                continue
            if buff_start < new_start:
                buff_start = new_start

        if new_end is not None:
            if buff_start >= new_end:
                continue
            if buff_end > new_end:
                buff_end = new_end

        trimmed.append(InBufferSpace(buff_start, buff_end, buff_space.buffer))

    return trimmed


def _merge_in_contiguous_space(overlaid_contiguous_spaces, buff):
    start = buff.start
    end = buff.end
    unsorted_spaces = [InBufferSpace(start, end, buff)]

    for sub_cs in overlaid_contiguous_spaces:
        if sub_cs.start < start:
            start = sub_cs.start
        if sub_cs.end > end:
            end = sub_cs.end

        if sub_cs.start == buff.end or buff.start == sub_cs.end:
            # Strictly contiguous
            trimmed = sub_cs.buffers
        elif sub_cs.start >= buff.start:
            if sub_cs.end <= buff.end:
                # Contiguous spaces totally overwritten by the new buffer
                continue
            else:
                trimmed = _trim_buffers(sub_cs.buffers, new_start=buff.end)
        else:
            if sub_cs.end > buff.end:
                # Contiguous space now split in two by the new buffer
                trimmed = _trim_buffers(sub_cs.buffers, new_start=buff.end)
                trimmed += _trim_buffers(sub_cs.buffers, new_end=buff.start)
            else:
                trimmed = _trim_buffers(sub_cs.buffers, new_end=buff.start)

        unsorted_spaces += trimmed

    return ContiguousSpace(start, end, sorted(unsorted_spaces))


def normalize_buffers(buffers):
    # Bruteforce mode: Insert one buffer after another and modify the
    # elements already present in the list if needed.
    # Should be fine enough for a small number of buffers.

    spaces = []
    start = inf
    end = -inf

    for buff in buffers:
        if not buff.size:
            continue

        if buff.start < start:
            start = buff.start
        if buff.end > end:
            end = buff.end

        # Retrieve the spaces our buffer overlaid. Those will be merged
        # together in a
        overlaid = [cs for cs in spaces if is_overlaid(cs, buff)]
        if overlaid:
            # All the overlaid spaces now constitute a contiguous space
            new_cs = _merge_in_contiguous_space(overlaid, buff)
            for to_remove in overlaid:
                spaces.remove(to_remove)
            bisect.insort(spaces, new_cs)
        else:
            # The buffer create a new contiguous space on it own
            ibs = InBufferSpace(buff.start, buff.end, buff)
            bisect.insort(spaces, ContiguousSpace(buff.start, buff.end, [ibs]))

    if not spaces:
        start = 0
        end = 0

    return UncontiguousSpace(start, end, spaces)
