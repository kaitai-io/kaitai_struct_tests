# Test for Python type annotations feature
# Validates that the Kaitai Struct compiler generates proper Python type annotations

import ast
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
import inspect
from integers import Integers
from expr_array import ExprArray
from nested_types import NestedTypes


class TestPythonTypeAnnotations(unittest.TestCase):
    """Test Python type annotation generation."""
    
    @classmethod
    def setUpClass(cls):
        """Set up test environment by checking compiled formats."""
        # Check if we can inspect the generated modules for type annotations
        pass

    def _check_imports(self, module, should_have_typing: bool = True):
        """Check that typing imports are present/absent as expected."""
        import inspect
        source = inspect.getsource(module)
        
        typing_import = "from typing import" in source
        type_ignore = "# type: ignore" in source
        
        if should_have_typing:
            self.assertTrue(typing_import, "Expected typing imports when annotations enabled")
            self.assertFalse(type_ignore, "Should not have '# type: ignore' when annotations enabled")
        else:
            self.assertFalse(typing_import, "Should not have typing imports when annotations disabled")
            self.assertTrue(type_ignore, "Should have '# type: ignore' when annotations disabled")

    def test_typing_imports_present(self):
        """Test that typing imports are present when annotations are enabled."""
        import integers
        self._check_imports(integers, should_have_typing=True)
    
    def test_constructor_annotations(self):
        """Test that constructor has proper type annotations."""
        import integers
        source = inspect.getsource(integers.Integers.__init__)
        
        # Look for constructor signature with type annotations
        self.assertIn("_io: 'KaitaiStream'", source)
        self.assertIn("-> None:", source)
    
    def test_primitive_type_annotations(self):
        """Test that primitive types have correct annotations."""
        import integers
        source = inspect.getsource(integers)
        
        # Check that we have typing imports
        self.assertIn("from typing import", source)
        
        # Check that we don't have type: ignore
        self.assertNotIn("# type: ignore", source)
    
    def test_array_type_annotations(self):
        """Test that array types have correct List[T] annotations."""
        import expr_array
        source = inspect.getsource(expr_array)
        
        # Should have List import
        self.assertIn("List", source)
        
        # Arrays should be initialized as empty lists
        self.assertIn("= []", source)
    
    def test_instance_property_annotations(self):
        """Test that instance properties have correct return type annotations."""
        import expr_array
        source = inspect.getsource(expr_array)
        
        # Look for property definitions with return types
        self.assertIn("@property", source)
        # Should have return type annotations
        self.assertIn("-> ", source)
    
    def test_user_type_annotations(self):
        """Test that user-defined types have correct annotations."""
        import nested_types
        source = inspect.getsource(nested_types)
        
        # Should have forward references for Kaitai types
        # Forward references should be quoted to handle circular dependencies
        self.assertTrue("'KaitaiStream'" in source or "'KaitaiStruct'" in source, 
                       "Forward references for Kaitai types missing")
    
    def test_method_return_annotations(self):
        """Test that methods have proper return type annotations."""
        import integers
        source = inspect.getsource(integers.Integers._read)
        
        # _read method should have -> None annotation
        self.assertRegex(source, r"def _read\(self\)\s*->\s*None:")
    
    def test_annotation_consistency(self):
        """Test that annotation presence is consistent across the file."""
        import integers
        source = inspect.getsource(integers)
        
        # Count function definitions with and without return annotations
        import re
        
        # Functions with return annotations
        annotated_functions = len(re.findall(r"def \w+\([^)]*\)\s*->\s*\w+:", source))
        
        # Total function definitions  
        total_functions = len(re.findall(r"def \w+\(", source))
        
        # Most functions should have return type annotations
        if total_functions > 0:
            annotation_ratio = annotated_functions / total_functions
            self.assertGreater(annotation_ratio, 0.5, 
                             f"Only {annotation_ratio:.1%} of functions have type annotations")
    
    def test_generated_code_is_valid_python(self):
        """Test that generated code with type annotations is syntactically valid."""
        modules = ["integers", "expr_array", "nested_types"]
        for module_name in modules:
            with self.subTest(module=module_name):
                try:
                    module = __import__(module_name)
                    source = inspect.getsource(module)
                    ast.parse(source)
                except ImportError:
                    self.skipTest(f"{module_name} module not available")
                except SyntaxError as e:
                    self.fail(f"Generated code has syntax error: {e}")


