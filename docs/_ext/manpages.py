# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
# cspell: words: astext, refdoc, anonlabels, refexplicit, refdomain, reftarget
from __future__ import annotations

import subprocess
from collections.abc import Callable
from functools import partial
from pathlib import Path
from types import NoneType

from docutils import nodes
from sphinx import addnodes
from sphinx.application import Sphinx
from sphinx.parsers import RSTParser
from sphinx.util.typing import ExtensionMetadata


class ManPageParser(RSTParser):
    supported = ("manpage",)

    def parse(self, input_string: str, document: nodes.document) -> None:
        command_prefixes = self._config.command_prefixes
        assert isinstance(command_prefixes, list)
        assert document.current_source is not None
        current_file = Path(document.current_source)
        document_id = make_id_from_path(current_file)

        assert isinstance(self._config.pandoc_path, Path | None)
        pandoc_path: str = (
            str(self._config.pandoc_path.resolve()) if self._config.pandoc_path else "pandoc"
        )
        # Convert the man page into RST
        rst_output = subprocess.check_output(
            args=[pandoc_path, "--from=man", "--to=rst"],
            input=input_string,
            encoding="utf-8",
        )

        # Let RSTParser do the heavy lifting
        super().parse(rst_output, document)

        title = get_document_title_from_synopsis(document, command_prefixes)

        remove_section_by_id(document, "version")
        main_section = wrap_in_section(document, title, document_id)
        remove_section_by_id(main_section, "name")

        subcommands = main_section.next_node(partial(check_section_id, id="subcommands"))
        if subcommands:
            assert isinstance(subcommands, nodes.section)
            remove_help_subcommands(subcommands)
            add_links_to_subcommands(subcommands, self._env.docname, command_prefixes)


def make_id_from_path(path: Path) -> str:
    return nodes.make_id(path.stem)


def get_document_title_from_synopsis(document: nodes.document, strip_prefixes: list[str]) -> str:
    synopsis = document.next_node(partial(check_section_id, id="synopsis"))
    assert isinstance(synopsis, nodes.section)
    paragraph = synopsis.next_node(nodes.paragraph)
    assert paragraph is not None
    command = paragraph.next_node(
        nodes.strong
    )  # The command is the first strong emphasized text block
    assert command is not None
    return make_title(command.astext(), strip_prefixes)


def make_title(base: str, strip_prefixes: list[str]) -> str:
    title = base
    if base not in strip_prefixes:
        for prefix in strip_prefixes:
            if base.startswith(prefix):
                title = base[len(prefix) + 1 :]  # +1 to include the `-` separator
                break
    return title.title()


def wrap_in_section(document: nodes.document, title: str, doc_slug: str) -> nodes.section:
    section_node = nodes.section()

    section_node["ids"] = [doc_slug]
    section_node["names"] = [doc_slug]  # Use the slug to identify the section

    title_node = nodes.title()
    title_node += nodes.Text(title)
    section_node += title_node

    existing_children = list(document.children)
    for child in existing_children:
        document.remove(child)
        section_node += child

    document += section_node
    document.note_explicit_target(section_node, section_node)

    document["title"] = title

    return section_node


def remove_section_by_id(document: nodes.Element, section_id: str) -> nodes.section | None:
    section = remove_node(document, partial(check_section_id, id=section_id))
    assert isinstance(section, nodes.section | None)
    return section


def check_section_id(node: nodes.Node, id: str) -> bool:
    return isinstance(node, nodes.section) and id in node["ids"]


def remove_node(
    document: nodes.Element, condition: Callable[[nodes.Node], bool]
) -> nodes.Node | None:
    node = document.next_node(condition)
    if node is not None:
        node.parent.remove(node)
        return node


def make_doc_ref(term: str, docname: str, refdoc: str) -> addnodes.pending_xref:
    ref_node = addnodes.pending_xref(
        "", refdomain="std", reftype="doc", reftarget=docname, refexplicit=True, refdoc=refdoc
    )
    ref_node += nodes.Text(term)
    return ref_node


def remove_help_subcommands(subcommands: nodes.section):
    entries = list(subcommands.findall(nodes.definition_list_item))
    for subcommand in entries:
        term = subcommand.next_node(nodes.term)
        assert term is not None
        if term.astext().endswith("-help(1)"):
            subcommand.parent.remove(subcommand)


def add_links_to_subcommands(subcommands: nodes.section, docname: str, strip_prefixes: list[str]):
    for raw_term in subcommands.findall(nodes.term):
        term, _man_index = raw_term.astext().split("(", maxsplit=1)
        target_id = nodes.make_id(term)
        title = make_title(term, strip_prefixes)

        ref = make_doc_ref(title, target_id, docname)

        raw_term.clear()  # Remove all children
        raw_term += ref


def setup(app: Sphinx) -> ExtensionMetadata:
    app.add_source_suffix(".1", "manpage")
    app.add_source_parser(ManPageParser)
    # To debug generated labels
    # app.connect("build-finished", debug_dump_labels)

    app.add_config_value(
        "pandoc_path",
        None,
        rebuild="",
        types=[
            Path,
            NoneType,
        ],
        description="""
        The path to pandoc executable, if not set will search for it in the current PATH
        """,
    )
    app.add_config_value(
        "command_prefixes",
        [],
        rebuild="env",
        types=list,
        description="""
        List of command prefixes, used to strip them from document title
        """,
    )

    return ExtensionMetadata(version="0.1", parallel_read_safe=True, parallel_write_safe=True)


# def debug_dump_labels(app: Sphinx, exception: Exception | None):
#     if exception is not None:
#         return
#     print(app.env.domaindata["std"]["labels"].keys())
#     print(app.env.domaindata["std"]["anonlabels"].keys())
