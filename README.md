<h2># 🔒 Password Manager</h2>

<p>Welcome to the **Password Manager** - a simple and secure command-line application to manage your passwords. Built with Rust, this tool helps you add, list, and search for your passwords effortlessly.</p>

<p>## 🌟 Features</p>

<li> **🔐 Add Entries**: Store new password entries with a service name, username, and password.</li>
<li> **📜 List Entries**: View all stored password entries in a clear format.</li>
<li> **🔍 Search Entries**: Quickly find a specific password entry by service name.</li>
<li> **🔐 Delete Entries**: Delete entry with a service name.</li>
<li> **🔐 Update Entries**: Update entry stored in a JSON file.</li>
<li> **🛡️ Secure Storage**: Passwords are securely stored in a JSON file.</li>
<br>
<p>## 🛠️ Prerequisites</p>

Ensure you have Rust installed on your machine. You can download it from [here](https://www.rust-lang.org/tools/install).

## 🚀 Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/password-manager.git
    ```

2. **Navigate to the project directory**:
    ```sh
    cd password-manager
    ```

3. **Build the project**:
    ```sh
    cargo build
    ```

## 📝 Usage

1. **Run the application**:
    ```sh
    cargo run
    ```

2. **Menu Options**:
    ```
    Password manager menu:
    1. Add Entry
    2. List Entries
    3. Search Entry
    4. Delete Entry
    5. Update Entry
    6. Exit Now
    ```

3. **Choose an option by entering the corresponding number**:
    - **1. Add Entry**: Follow the prompts to add a new entry.
    - **2. List Entries**: View all stored entries.
    - **3. Search Entry**: Search for an entry by service name.
    - **4. Delete Entry**: Delete the stored entry.
    - **5. Update Entry**: Update stored entry.
    - **6. Exit Now**: Exit the application.
