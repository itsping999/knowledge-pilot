"""Answer generation interfaces and implementations."""

from knowledge_pilot.generation.base import AnswerGenerator
from knowledge_pilot.generation.stub import ExtractiveAnswerGenerator

__all__ = ["AnswerGenerator", "ExtractiveAnswerGenerator"]

