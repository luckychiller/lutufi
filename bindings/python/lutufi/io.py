"""I/O operations module.

This module provides input/output operations for various network formats:
- GraphML (read/write)
- GEXF (read/write)
- Pajek/NET (read/write)
- GML (read/write)
- Edge list (CSV/TSV)
- JSON adjacency list
- Adjacency matrix
"""

from typing import Optional, Union, Dict, Any, Literal
from pathlib import Path


FileFormat = Literal[
    "graphml", "gexf", "pajek", "net", "gml",
    "edgelist", "adjlist", "adjmatrix", "json", "bin"
]


class GraphIOError(Exception):
    """Error raised during graph I/O operations."""
    pass


def _import_networkx():
    """Lazy-import networkx with a helpful error message."""
    try:
        import networkx as nx
        return nx
    except ImportError:
        raise ImportError(
            "networkx is required for graph I/O operations.\n"
            "Install it with: pip install networkx"
        )


def read_graph(
    path: Union[str, Path],
    format: Optional[FileFormat] = None,
    **kwargs: Any,
) -> Any:
    """Read a graph from a file.

    Delegates to NetworkX's graph reading functions for supported formats.
    Returns a NetworkX Graph/DiGraph. Use BayesianNetwork.from_networkx() to
    convert to a Lutufi model.

    Args:
        path: Path to the file.
        format: File format (inferred from extension if not specified).
        **kwargs: Additional format-specific options.

    Returns:
        A NetworkX graph object.

    Raises:
        GraphIOError: If the file cannot be read or format is unsupported.

    Example:
        >>> G = read_graph("network.graphml")
        >>> bn = BayesianNetwork.from_networkx(G)
    """
    nx = _import_networkx()
    path = Path(path)

    if format is None:
        format = _infer_format(path.suffix)

    readers = {
        "graphml": nx.read_graphml,
        "gexf": nx.read_gexf,
        "gml": nx.read_gml,
        "pajek": nx.read_pajek,
        "edgelist": nx.read_edgelist,
        "adjlist": nx.read_adjlist,
    }

    reader = readers.get(format)
    if reader is None:
        raise GraphIOError(f"Unsupported format: {format}")

    try:
        return reader(str(path), **kwargs)
    except Exception as e:
        raise GraphIOError(f"Failed to read {path} as {format}: {e}")


def write_graph(
    graph: Any,
    path: Union[str, Path],
    format: Optional[FileFormat] = None,
    **kwargs: Any,
) -> None:
    """Write a graph to a file.

    Delegates to NetworkX's graph writing functions for supported formats.

    Args:
        graph: NetworkX graph or Lutufi model to write.
        path: Path to the output file.
        format: File format (inferred from extension if not specified).
        **kwargs: Additional format-specific options.

    Raises:
        GraphIOError: If the file cannot be written or format is unsupported.

    Example:
        >>> write_graph(model.to_networkx(), "output.graphml")
    """
    nx = _import_networkx()
    path = Path(path)

    if format is None:
        format = _infer_format(path.suffix)

    # Convert Lutufi models to NetworkX if needed
    if hasattr(graph, "to_networkx"):
        graph = graph.to_networkx()

    graph = _stringify_list_attrs(graph, nx)

    writers = {
        "graphml": nx.write_graphml,
        "gexf": nx.write_gexf,
        "gml": nx.write_gml,
        "pajek": nx.write_pajek,
        "edgelist": nx.write_edgelist,
        "adjlist": nx.write_adjlist,
    }

    writer = writers.get(format)
    if writer is None:
        raise GraphIOError(f"Unsupported format: {format}")

    try:
        writer(graph, str(path), **kwargs)
    except Exception as e:
        raise GraphIOError(f"Failed to write {path} as {format}: {e}")


def _stringify_list_attrs(graph: Any, nx: Any) -> Any:
    """Return a copy of `graph` with list/tuple-valued attributes converted
    to delimited strings.

    Formats like GraphML and GEXF only support scalar attribute types, so
    list-valued attributes (e.g. a variable's ``domain``) must be flattened
    to strings before writing.
    """
    has_list_attr = any(
        isinstance(v, (list, tuple))
        for _, data in graph.nodes(data=True)
        for v in data.values()
    ) or any(
        isinstance(v, (list, tuple))
        for _, _, data in graph.edges(data=True)
        for v in data.values()
    )
    if not has_list_attr:
        return graph

    graph = graph.copy()
    for _, data in graph.nodes(data=True):
        for key, value in data.items():
            if isinstance(value, (list, tuple)):
                data[key] = "|".join(str(v) for v in value)
    for _, _, data in graph.edges(data=True):
        for key, value in data.items():
            if isinstance(value, (list, tuple)):
                data[key] = "|".join(str(v) for v in value)
    return graph


def _infer_format(suffix: str) -> FileFormat:
    """Infer file format from extension.

    Args:
        suffix: File extension (including the dot).

    Returns:
        File format string.

    Raises:
        GraphIOError: If the format cannot be inferred.
    """
    mapping: Dict[str, FileFormat] = {
        ".graphml": "graphml",
        ".gexf": "gexf",
        ".net": "pajek",
        ".paj": "pajek",
        ".gml": "gml",
        ".txt": "edgelist",
        ".csv": "edgelist",
        ".tsv": "edgelist",
        ".adj": "adjlist",
        ".json": "json",
    }

    ext = suffix.lower()
    if ext not in mapping:
        raise GraphIOError(f"Cannot infer format from extension: {suffix}")

    return mapping[ext]


class GraphReader:
    """Reader for graph files.

    Provides a class-based interface for reading graphs with
    customizable options.

    Example:
        >>> reader = GraphReader(format="graphml")
        >>> graph = reader.read("network.graphml")
    """

    def __init__(
        self,
        format: FileFormat,
        **options: Any,
    ):
        """Initialize the reader.

        Args:
            format: File format to read.
            **options: Format-specific options.
        """
        self._format = format
        self._options = options

    def read(self, path: Union[str, Path]) -> Any:
        """Read a graph from a file.

        Args:
            path: Path to the file.

        Returns:
            Graph object.
        """
        return read_graph(path, format=self._format, **self._options)


class GraphWriter:
    """Writer for graph files.

    Provides a class-based interface for writing graphs with
    customizable options.

    Example:
        >>> writer = GraphWriter(format="graphml")
        >>> writer.write(graph, "output.graphml")
    """

    def __init__(
        self,
        format: FileFormat,
        **options: Any,
    ):
        """Initialize the writer.

        Args:
            format: File format to write.
            **options: Format-specific options.
        """
        self._format = format
        self._options = options

    def write(
        self,
        graph: Any,
        path: Union[str, Path],
    ) -> None:
        """Write a graph to a file.

        Args:
            graph: Graph object to write.
            path: Path to the output file.
        """
        write_graph(graph, path, format=self._format, **self._options)
