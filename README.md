# 🎬 Jellynizer

Jellynizer is a powerful, automated media organization tool designed to scan your media collections, identify files (movies and TV shows), and organize them into a clean, standardized directory structure.

It features a modern web interface for previewing changes before they are applied, giving you full control over your media library.

---

## ✨ Features

- 🔍 **Smart Scanning**: Recursively scans specified source directories for media files.
- 🏷️ **Metadata Enrichment**: Automatically identifies media type (Movie vs. Show) and enriches file data.
- 🛡️ **Preview Mode**: See exactly which files will be moved or copied before committing to any changes.
- ⚙️ **Flexible Operations**: Supports both `move` (default) and `copy` modes.
- 🤖 **Automatic Mode**: Configure Jellynizer to run organization automatically after every scan for hands-free organization.
- 🌐 **Web Interface**: A clean, intuitive React-based dashboard to manage your library.
- 🐳 **Containerized**: Fully Docker-ready for easy deployment.

---

## 🚀 Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### 🚀 Running with Docker

You can run Jellynizer using a single `docker run` command or via `docker-compose`.

#### Option 1: Using `docker run` (Quickest)
Run the container directly with the following command:
```bash
docker run -d \
  -p 3000:3000 \
  -v /path/to/source:/data/source \
  -v /path/to/movies:/data/movies \
  -v /path/to/shows:/data/shows \
  --name jellynizer \
  alith/jellynizer:latest
```

#### Option 2: Using `docker-compose` (Recommended)
The easiest way to manage configuration is using Docker Compose.

1. **Create a `docker-compose.yml` file**:
   ```yaml
   version: '3.8'
   services:
     jellynizer:
       image: alith/jellynizer:latest
       container_name: jellynizer
       ports:
         - "3000:3000"
       environment:
         - SOURCE_DIRS=/data/source
         - DEST_MOVIES=/data/movies
         - DEST_SHOWS=/data/shows
       volumes:
         - ./media:/data/source
         - ./movies:/data/movies
         - ./shows:/data/shows
       restart: always
   ```

2. **Run the application**:
   ```bash
   docker-compose up -d
   ```

3. **Access the Dashboard**:
   Open your browser and navigate to `http://localhost:3000`.

---

## 🛠 Configuration

Jellynizer is configured via environment variables.

| Variable | Description | Default |
| :--- | :--- | :--- |
| `SOURCE_DIRS` | Comma-separated list of directories to scan. | `.` |
| `DEST_MOVIES` | Destination directory for movie files. | `./movies` |
| `DEST_SHOWS` | Destination directory for TV show files. | `./shows` |
| `OPERATION` | The action to take: `move` or `copy`. | `move` |
| `RUN_AUTOMATICALLY`| If `true`, organization happens immediately after scan. | `false` |

### Example `.env` file for Docker Compose:
```env
SOURCE_DIRS=/mnt/nas/downloads
DEST_MOVIES=/mnt/nas/media/movies
DEST_SHOWS=/mnt/nas/media/shows
OPERATION=move
RUN_AUTOMATICALLY=false
```

---

## 📖 Usage Guide

### Using the Web Interface

1. **Scan**: Click the **"Scan"** button. Jellynizer will traverse your source directories and identify media files.
2. **Review**: Once the scan is complete, the **"Preview"** section will populate with a list of proposed changes (e.g., `old_path/file.mkv` $\rightarrow$ `movies/Movie Name (Year)/Movie Name (Year).mkv`).
3. **Execute**: 
   - If you are satisfied, click **"Execute"** to perform the moves/copies.
   - If you want to automate this, set `RUN_AUTOMATICALLY=true` in your configuration.

### API Reference (For Developers)

Jellynizer provides a RESTful API built with Axum.

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `POST` | `/api/scan` | Triggers a background media scan. |
| `GET` | `/api/preview` | Returns the list of proposed file moves. |
| `POST` | `/api/execute` | Executes the proposed moves/copies. |
| `GET` | `/api/media` | Retrieves the list of all discovered media. |
| `GET` | `/api/status` | Returns current scanning status and configuration. |

---

## 📂 Project Structure

```text
jellynizer/
├── src/                # Rust Backend
│   ├── api/            # REST API implementation
│   ├── organizer/      # Core logic for moving/copying files
│   ├── scanner/        # Directory traversal logic
│   ├── metadata/       # Media identification & enrichment
│   └── main.rs         # Application entry point
├── frontend/           # React/Vite Frontend
│   ├── src/            # UI Components and logic
│   └── dist/           # Production build assets
├── Dockerfile          # Multi-stage Docker build
├── docker-compose.yml  # Orchestration for easy deployment
└── Cargo.toml          # Rust dependencies
```

---

## 🛡 License

[Specify your license here, e.g., MIT]