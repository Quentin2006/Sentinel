# Sentinel

> A compiler-agnostic AI-assisted build copilot that fixes compilation errors automatically.

Sentinel is a compiler-agnostic CLI wrapper built in Rust that uses AI to fix compilation errors. Run your normal compiler (or any build system that outputs compiler messages) through Sentinel, and if the build fails, it will apply fixes to make it compile.

## Installation

```bash
# NOTE: this tool requires opencode to be installed and runnable via the 'opencode'
# command, for more info on how to install on your system, look at https://opencode.ai/

git clone https://github.com/Quentin2006/Sentinel.git
cd Sentinel
cargo install --path .

# it will now be installed in
~/.cargo/bin/ai-sentinel
# but I recommend adding an alias such as the following
alias sen="~/.cargo/bin/ai-sentinel"

# in the rest of this file I will be using my alias
```

## Quick Start

Run your normal compiler through Sentinel:

```bash
sen g++ main.cpp
sen gcc main.c -o main
sen clang++ src/main.cpp -o app
sen make
```

## Example

```bash
sen make
  . [1/2] Compiling⚠ compilation failed, attempting fix...
  src/gen.cpp: In function ‘std::vector<ObjectConfig> genLightsForCoaster(const std::vector<glm::vec<3, float, glm::packed_highp> >&, int, glm::vec3, float)’:
  src/gen.cpp:17:44: warning: unused parameter ‘phase’ [-Wunused-parameter]
    17 |                     glm::vec3 color, float phase) {
        |                                      ~~~~~~^~~~~
  src/mesh.cpp: In member function ‘int Mesh::loadSweep(const std::vector<glm::vec<3, float, glm::packed_highp> >&, int, int, float)’:
  src/mesh.cpp:347:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    347 |       vertices.push_back({p1, glm::vec2(0.0f), normal1});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:348:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    348 |       vertices.push_back({p0, glm::vec2(0.0f), normal1});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:349:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    349 |       vertices.push_back({p2, glm::vec2(0.0f), normal1});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:355:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    355 |       vertices.push_back({p2, glm::vec2(0.0f), normal2});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:356:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    356 |       vertices.push_back({p0, glm::vec2(0.0f), normal2});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:357:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    357 |       vertices.push_back({p3, glm::vec2(0.0f), normal2});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:374:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    374 |       vertices.push_back(
        |       ~~~~~~~~~~~~~~~~~~^
    375 |           {circles[0][next_j], glm::vec2(0.0f), -tangentAtStart});
        |           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:376:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    376 |       vertices.push_back({circles[0][j], glm::vec2(0.0f), -tangentAtStart});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:377:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    377 |       vertices.push_back({smoothPath[0], glm::vec2(0.0f), -tangentAtStart});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:387:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    387 |       vertices.push_back({smoothPath[last], glm::vec2(0.0f), tangentAtEnd});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:388:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    388 |       vertices.push_back({circles[last][j], glm::vec2(0.0f), tangentAtEnd});
        |       ~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/mesh.cpp:389:25: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    389 |       vertices.push_back(
        |       ~~~~~~~~~~~~~~~~~~^
    390 |           {circles[last][next_j], glm::vec2(0.0f), tangentAtEnd});
        |           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  src/app.cpp: In member function ‘void App::run()’:
  src/app.cpp:229:68: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    229 |   terrainV1 = {{0.0f, 0.0f, 0.0f}, {0.0f, 0.0f}, {0.0f, 1.0f, 0.0f}};
        |                                                                    ^
  src/app.cpp:230:68: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    230 |   terrainV2 = {{0.0f, -1.f, 3.0f}, {1.0f, 0.0f}, {0.0f, 1.0f, 0.0f}};
        |                                                                    ^
  src/app.cpp:231:68: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    231 |   terrainV3 = {{5.0f, 0.3f, 0.0f}, {0.0f, 1.0f}, {0.0f, 1.0f, 0.0f}};
        |                                                                    ^
  src/fractal_terrain.cpp: In function ‘Vertex midVertex(Vertex&, Vertex&)’:
  src/fractal_terrain.cpp:58:24: warning: missing initializer for member ‘Vertex::materialId’ [-Wmissing-field-initializers]
    58 |   return {pos, texCoord};
        |                        ^
  .   [2/2] Applying fix✓ fix applied successfully
  Now I understand the issues. I need to:
  1. Add a default value for `materialId` in the Vertex struct (fixes all initializer warnings)
  2. Fix the unused parameter `phase` in gen.cpp
  Build successful with no warnings. Fixed all 18 warnings with 2 minimal changes:

  1. **`src/vertexBuffer.h`**: Added default value `= 0` to `materialId` - fixes all 17 missing initializer warnings
  2. **`src/gen.cpp:17`**: Changed unused `phase` parameter to anonymous `float` - fixes unused parameter warning
```

## How It Works

Sentinel wraps your compiler and handles errors:

1. **Invocation**: Run your compiler through Sentinel (e.g., `sen gcc ...`).
2. **Capture**: Sentinel intercepts all compiler output.
3. **Fix**: If compilation fails, Sentinel will prompt opencode to fix the errors.
4. **Validate**: Opencode recompiles to verify the fix works.

The loop continues until compilation succeeds or opencode gives up.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome once the project reaches a stable state. Check back later for contribution guidelines.
