# Django Tools CLI

Django Tools CLI is a command-line interface (CLI) tool written in Rust 🦀 to quickly create APIs in Django. It provides commands to generate different components of a Django project, such as ViewSet, ApiView, and new projects.

## Installation
1. **Build the project**: You can build the project from the source code. Clone the repository and navigate to the project directory. Then, run the following command to build the project:

    ```
    cargo build --release
    ```

2. **Configure depending on the operating system**:

    - **Windows**:
        - Take the generated `.exe` file, for example, `django-tools.exe`, and place it in a directory of your choice. For instance, you can place it in `C:\Program Files`.
        - Copy the entire directory path where the `.exe` file is located.
        - Add the copied directory path to the system's environment variables. To do this:
            - Search for "Environment Variables" in the Windows search bar and open the system's environment variables settings.
            - In the System Properties window, click on the "Environment Variables" button.
            - Under "System variables", find the "Path" variable, select it, and click on "Edit".
            - Click on "New" and paste the directory path where the `.exe` file is located.
            - Click "OK" on all windows to save the changes.

    - **Linux**:
        - Take the generated binary file (executable), typically located at `target/release/django-tools` after building the project.
        - Move or copy the binary file to `/usr/local/bin` directory using the following command:
            ```
            sudo cp target/release/django-tools /usr/local/bin/
            ```
        - You might need to use `sudo` to have the necessary permissions to copy the file.

## Usage

You can utilize Django Tools CLI with the following commands:

### Creating a new project:

To initiate a new Django project, use the following command:
```bash
django-tools Project -n backend
```
This command will create a new Django project named "backend".

### Creating a ViewSet app:
To generate a ViewSet app within your Django project, use the following command:
```bash
django-tools ViewSet -n ducks
```
This command will generate a ViewSets app named "ducks" within your Django project.

### Creating an ApiView app:

To generate an ApiView app within your Django project, use the following command:
```bash
django-tools ApiView -n dogs
```
This command will create an ApiView app named "dogs" within your Django project.


![Captura de pantalla 2024-04-27 184334](https://github.com/xansaul/django-tools/assets/90731443/f135967b-59bd-437a-8160-69a31058f2a8)


