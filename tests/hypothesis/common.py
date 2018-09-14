import os
import re
import attr
from functools import wraps
from hypothesis.stateful import (
    RuleBasedStateMachine,
    rule as vanilla_rule,
    initialize as vanilla_initialize,
    VarReference,
)
from huepy import red, bold


def rule(**config):
    def dec(fn):
        @vanilla_rule(**config)
        @wraps(fn)
        def wrapper(*args, **kwargs):
            print(red(bold("%s(%s)" % (fn.__name__, kwargs))))
            return fn(*args, **kwargs)

        return wrapper

    return dec


def initialize(**config):
    def dec(fn):
        @vanilla_initialize(**config)
        @wraps(fn)
        def wrapper(*args, **kwargs):
            print(red(bold("%s(%s)" % (fn.__name__, kwargs))))
            return fn(*args, **kwargs)

        return wrapper

    return dec


class FileOracle:
    def __init__(self):
        self._buffer = bytearray()

    def read(self, size, offset):
        return self._buffer[offset : size + offset]

    def write(self, offset, content):
        if not content:
            return
        gap = offset - len(self._buffer)
        if gap > 0:
            self._buffer += bytearray(gap)
        self._buffer[offset : len(content) + offset] = content

    def truncate(self, length):
        new_buffer = bytearray(length)
        copy_size = min(length, len(self._buffer))
        new_buffer[:copy_size] = self._buffer[:copy_size]
        self._buffer = new_buffer


@attr.s
class OracleFSFile:
    need_sync = attr.ib(default=True)
    base_version = attr.ib(default=0)

    @property
    def is_placeholder(self):
        return self.base_version == 0


@attr.s(repr=False)
class OracleFSFolder(dict):
    need_sync = attr.ib(default=True)
    base_version = attr.ib(default=0)

    def __repr__(self):
        children = {k: v for k, v in self.items()}
        return (
            "{type}("
            "need_sync={self.need_sync}, "
            "base_version={self.base_version}, "
            "children={children})"
        ).format(type=type(self).__name__, self=self, children=children)

    @property
    def is_placeholder(self):
        return self.base_version == 0


def normalize_path(path):
    return "/" + "/".join([x for x in path.split("/") if x])


class BaseFailureReproducer:

    _FAILURE_REPRODUCER_CODE_HEADER = """import pytest


def test_reproduce(alice_core_sock, alice2_core2_sock):
"""

    async def teardown(self):
        super().teardown()
        if hasattr(self, "_failure_reproducer_code"):
            reproduce_code = self._failure_reproducer_template.format(
                body="\n".join(self._failure_reproducer_code).strip()
            )
            reproduce_file = os.environ.get("REPRODUCE_FILE")
            if reproduce_file:
                with open(reproduce_file, "w") as fd:
                    fd.write(reproduce_code)
                print(f"========>>> Reproduce code available at {reproduce_file} <<<=======")
            else:
                print("============================ REPRODUCE CODE ========================")
                print(reproduce_code)
                print("====================================================================")

    def print_step(self, step):
        super().print_step(step)

        if not hasattr(self, "_failure_reproducer_code"):
            assert "{body}" in self._FAILURE_REPRODUCER_CODE_HEADER
            self._failure_reproducer_template = self._FAILURE_REPRODUCER_CODE_HEADER
            self._failure_reproducer_code = []
            match = re.search(
                r"^( *)\{body\}", self._failure_reproducer_template, flags=re.MULTILINE
            )
            self._failure_reproducer_indent = match.group(1)

        def indent(code):
            if not isinstance(code, list):
                code = code.strip().split("\n")
            return [self._failure_reproducer_indent + x for x in code]

        rule, data = step
        template = getattr(rule.function, "_reproduce_template", None)
        rule_brief = "%s(%r)" % (rule.function.__name__, data)
        if not template:
            self._failure_reproducer_code += indent("# Nothing to do for rule %s" % rule_brief)
            return
        else:
            self._failure_reproducer_code.append("")
            self._failure_reproducer_code += indent("# Rule %s" % rule_brief)

        resolved_data = {}
        for k, v in data.items():
            if isinstance(v, VarReference):
                resolved_data[k] = repr(self.names_to_values[v.name])
            else:
                resolved_data[k] = repr(v)

        code = template.format(**resolved_data)
        self._failure_reproducer_code += indent(code)


def failure_reproducer(init=BaseFailureReproducer._FAILURE_REPRODUCER_CODE_HEADER):
    def wrapper(cls):
        assert issubclass(cls, RuleBasedStateMachine)
        nmspc = {"_FAILURE_REPRODUCER_CODE_HEADER": init}
        return type("%sWithFailureReproducer" % cls.__name__, (BaseFailureReproducer, cls), nmspc)

    return wrapper


def reproduce_rule(template):
    def wrapper(fn):
        fn._reproduce_template = template
        return fn

    return wrapper
