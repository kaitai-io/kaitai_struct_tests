# Validation script for Python type annotations

import inspect


def validate_type_annotations():
    """Validate that generated type annotations are correct."""
    
    # Test integers module
    import integers
    source = inspect.getsource(integers)
    
    # Check imports
    assert "from typing import" in source, "No typing imports found"
    
    # Check constructor
    init_source = inspect.getsource(integers.Integers.__init__)
    assert "_io: 'KaitaiStream'" in init_source, "Constructor missing KaitaiStream type annotation"
    assert "-> None:" in init_source, "Constructor missing return type annotation"
    
    # Test expr_array module  
    import expr_array
    source = inspect.getsource(expr_array)
    
    # Check for List type usage
    assert "List" in source, "List type annotations missing"
    
    # Check for property annotations
    assert "@property" in source and "-> " in source, "Property return type annotations missing"
    
    # Test nested_types module
    import nested_types
    source = inspect.getsource(nested_types)
    
    # Check for forward references in type annotations
    assert "'KaitaiStream'" in source or "'KaitaiStruct'" in source, "Forward references for Kaitai types missing"


if __name__ == '__main__':
    validate_type_annotations()