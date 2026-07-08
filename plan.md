# Jellynizer Project Plan

## Project Overview
Jellynizer is a Rust-based application designed to organize media files (Movies and TV Shows) to comply with Jellyfin's preferred directory structure. The goal is to provide a low-resource, highly optimized tool that can run as a background containerized service, while providing a UI for monitoring and reviewing organization actions.

## Core Requirements
- **Low Resource Consumption**: Leveraging Rust for high performance and minimal footprint.
- **Dockerized**: Optimized for running in a Docker container.
- **Background Task Capability**: Designed to run as a daemon or background service.
- **Jellyfin Compliance**: Strictly follows Jellyfin's media organization guidelines.
- **User Interface**: A UI to visualize and verify the file organization process.
- **Consistent Logging**: Structured, consistent logging throughout the application for observability and debugging.
- **Metadata Enrichment**: Capability to fetch metadata via web scraping/API calls to ensure accurate naming and folder structures.

## Technical Specifications

### 1. Media Organization Logic
- **Input**: Configuration specifying source folders and destination folders per category.
- **Processing**:
    - Parse source directory structures and filenames.
    - **Metadata Fetching**: Integrate web scraping or API clients (e.g., TMDB, IMDb) to verify file identity and fetch correct titles/years.
    - Identify media types (Movie vs. TV Show) using filename patterns and fetched metadata.
    - Map files to the correct Jellyfin structure:
        - **Movies**: `Movies/Movie Name (Year)/Movie Name (Year).ext`
        - **Shows**: `Shows/Show Name/Season XX/Show Name - SXXEYY - Episode Name.ext`

### 2. Architecture
- **Backend (Rust)**:
    - **Web Framework**: **Axum** (chosen for its performance, modularity, and excellent integration with the `tower` ecosystem).
    - File system crawler (optimized for large directories).
    - Pattern matching engine (Regex/Metadata).
    - Metadata client (Web scraping/API).
    - File mover/organizer (atomic moves).
    - API server (Axum) to communicate with the UI.
- **Frontend (UI)**:
    - **Framework**: **React** with **Tailwind CSS** (chosen for rapid development, rich component ecosystem, and excellent compatibility with a REST/JSON API provided by Axum).
    - Web-based interface to view source/destination mappings.
    - Preview mode (Dry run) to show what *would* happen before moving files.
    - Logs/Status view.
- **Containerization**:
    - Multi-stage Dockerfile for minimal image size.
    - Volume mapping for source and destination directories.

### 3. Planned Workflow
1. **Scan**: Recursively walk through configured source directories.
2. **Analyze & Enrich**: Match files against patterns and fetch metadata to confirm identity.
3. **Dry Run (UI)**: Present the proposed changes to the user via the UI.
4. **Execute**: Move/Rename files to the destination based on user approval or automated rules.

## Implementation Roadmap

### Phase 1: Foundation & CLI (Backend)
- [ ] Initialize Rust project.
- [ ] Implement structured logging (e.g., `tracing` crate).
- [ ] Implement directory walking and file metadata extraction.
- [ ] Implement pattern matching logic for Movies and TV Shows.
- [ ] Implement basic CLI for file organization (Dry run vs. Execute).
- [ ] **Unit Testing**: Implement unit tests for all core logic (pattern matching, path generation).

### Phase 2: Metadata & Enrichment
- [ ] Implement web scraping/API client for metadata retrieval.
- [ ] Integrate metadata into the file analysis pipeline.
- [ ] **Integration Testing**: Verify metadata retrieval and file mapping accuracy.

### Phase 3: UI & API
- [ ] Implement lightweight API server in Rust using **Axum**.
- [ ] Develop Frontend (React) to interact with the API.
- [ ] Implement "Preview" functionality in the UI.

### Phase 4: Containerization
- [ ] Create optimized multi-stage Dockerfile.
- [ ] Define environment variables for configuration (Source/Destination paths).

### Phase 5: Optimization & Refinement
- [ ] Optimize file scanning for high-capacity storage.
- [ ] Ensure background task stability.
- [ ] Final end-to-end testing against Jellyfin structure requirements.