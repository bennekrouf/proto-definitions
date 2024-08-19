
# Proto Definitions Repository

## Overview

This repository contains all the Protocol Buffer (`.proto`) files utilized for communication between various components in our system, whether acting as a client or server. By centralizing these proto files, we ensure consistency across all microservices and clients, enabling seamless integration and communication.

## Getting Started

### Cloning the Repository

To include this repository in your project as a submodule, follow these steps:

1. **Add the Submodule:**

   From the root of your project, add the proto-definitions repository as a submodule:

   ```bash
   git submodule add git@192.95.30.129:backend/proto-definitions.git


2. Initialize and Update the Submodule:

After adding the submodule, you need to initialize and fetch its contents:

```bash
    git submodule init
    git submodule update


### Updating the Proto Files

Proto files in this repository may be updated over time. To ensure your project always uses the latest version, follow these steps:

1. Fetch the Latest Changes:

    To update the proto files to their latest version, run:


```bash
git submodule update --remote



If not working force with :


rm -Rf cached proto-definitions
git rm --cache proto-definitions
git submodule add git@192.95.30.129:backend/proto-definitions.git

2. Commit the Changes:

After updating the submodule, ensure you commit the changes to your project’s repository:


```bash
git add proto-definitions
git commit -m "Updated proto definitions to the latest version"


## Usage

Once you've added and initialized the submodule, the proto files are available within your project's directory. You can now use them in your build pipeline, such as generating code for your services or clients using a Protocol Buffer compiler (e.g., protoc).

### Example: Generating Code

To generate code from the proto files, you can typically run a command like the following (assuming protoc is installed and your build environment is configured):


```bash
protoc --proto_path=proto-definitions --<language_out>=<output_directory> proto-definitions/**/*.proto


Replace <language_out> and <output_directory> with your specific target language and desired output directory.



## Contributing

If you need to update the proto files:

    Make the necessary changes in a new branch.
    Push your changes and open a pull request for review.
    Once approved, merge your changes into the main branch.

## Updating the Main Repo After Merging

After merging changes into the proto-definitions repository, ensure all dependent projects update their submodule to reflect the latest proto definitions by following the update steps outlined above.
