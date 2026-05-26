"""Document loaders."""

from __future__ import annotations

from pathlib import Path

from knowledge_pilot.schema import Document


def load_text_file(path: Path) -> Document:
    """Load a UTF-8 text file as a document."""

    resolved = path.expanduser().resolve()
    return Document(
        id=resolved.stem,
        text=resolved.read_text(encoding="utf-8"),
        metadata={"source": str(resolved), "title": resolved.name},
    )

