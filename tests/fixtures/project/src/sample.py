"""
Sample Python module for testing file operations.
"""


def greet(name: str) -> str:
    """Return a greeting message."""
    return f"Hello, {name}!"


def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b


def process_data(data: list[int]) -> dict:
    """Process a list of integers and return statistics."""
    if not data:
        return {"count": 0, "sum": 0, "mean": 0}

    return {
        "count": len(data),
        "sum": sum(data),
        "mean": sum(data) / len(data),
        "min": min(data),
        "max": max(data),
    }


if __name__ == "__main__":
    print(greet("World"))
    print(add(2, 3))
    print(process_data([1, 2, 3, 4, 5]))
