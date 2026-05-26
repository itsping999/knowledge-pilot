"""Provider-neutral tool protocol."""

from __future__ import annotations

from typing import Protocol

from knowledge_pilot.schema import ToolResult


class Tool(Protocol):
    """Callable tool that can be used by future agents."""

    name: str
    description: str

    def run(self, query: str) -> ToolResult:
        """Run the tool for a query."""

