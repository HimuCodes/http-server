Rust File Server

This project implements a basic HTTP file server written in Rust. It allows clients to retrieve files from a specified directory on the server.

Learning Journey

I embarked on learning Rust using The Book. While the book provided a solid foundation, I encountered moments of confusion and discouragement. 
To overcome these hurdles, I sought assistance from Code Crafters, a valuable resource that helped me grasp the concepts and apply them in this project.
There were also a lot of problems which I faced while doing this so I also seeked assistance from AI assistants which explained step by step process to me.

Rust has a very hard learning curve and I can see why people say that, but slowly and steadily I am getting there. Along with this I also know that the book's last lesson is to create a web server but I thought it would be easy if I tried on my own.
I can tell you it was not but I tried and I'll keep trying till I get there.

I am also parrallely following Let's Get Rusty's Bootcamp to learn Rust.
I guess we'll meet on the next project then.

Code Breakdown

The code leverages the following standard library modules:

    std::fs: Provides functionalities for file system operations like opening and reading files.
    std::io: Handles input and output operations, including reading from and writing to network streams.
    std::net: Enables networking capabilities for creating TCP listeners and handling connections.
    std::path: Facilitates working with file paths and manipulating them.

Explanation

    Command-Line Arguments:
        The code retrieves the directory path from command-line arguments.
        If no argument is provided, it prints usage instructions and exits.
        It validates the arguments to ensure they are in the correct format (e.g., --directory /path/to/directory).

    Server Setup:
        A TCP listener is bound to the local loopback address (127.0.0.1) on port 4221. This allows clients on the same machine to connect to the server.
        The code enters an infinite loop to continuously accept incoming connections.

    Handling Connections:
        The server iterates over incoming connections from the listener.
        For each connection:
            If successful, it reads data from the client using a buffer ([0; 1024]).
            The received data is interpreted as a UTF-8 encoded string (String::from_utf8_lossy).
            The request is parsed, extracting the HTTP method (e.g., GET), path (/files/filename), and HTTP version.

    Serving Files:
        If the request method is GET and the path starts with /files/, the code attempts to serve the requested file.
        It extracts the filename from the path and constructs the full file path based on the provided directory argument.
        The file is opened using File::open. If successful:
            The file contents are read into a vector (Vec::new()).
            A response message is constructed, including the HTTP status code (200 OK), content type (application/octet-stream), content length (size of the file), and the actual file contents.
            The response is sent back to the client using stream.write_all.

    Handling Errors:
        If the file cannot be opened (e.g., file not found), a 404 Not Found response is sent to the client.
        Any errors encountered during communication or file operations are printed to the console.

Running the File Server

    Compile the code into a binary executable using a Rust compiler (e.g., cargo build).

    Run the server binary, specifying the directory containing the files you want to serve:
    
    ./your_server_binary_name --directory /path/to/your/directory


Use an HTTP client (e.g., browser or curl) to access files by making GET requests to the server's URL followed by the file path (e.g., http://localhost:4221/files/myfile.txt).
