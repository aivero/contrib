#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Conan recipe package for Google FlatBuffers
"""
import os
import shutil
from build import *


class FlatbuffersConan(Recipe):
    settings = Recipe.settings + ("compiler",)
    license = "Apache"
    description = "Memory Efficient Serialization Library"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.15.3]")

    def source(self):
        self.get(f"https://github.com/google/flatbuffers/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        defs = {
            "FLATBUFFERS_BUILD_TESTS": False,
            "FLATBUFFERS_BUILD_SHAREDLIB": self.options.shared,
            "FLATBUFFERS_BUILD_FLATLIB": not self.options.shared,
        }
        self.cmake(defs)
