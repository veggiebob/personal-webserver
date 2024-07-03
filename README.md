
# Setup

Follow directions in [this article](https://www.mwells.org/coding/2016/authbind-port-80-443/v) 
to properly use authbind for port 80



# Running the server

Easiest way to run the server is by running

(For the first time)
```bash
./start-server --init
```

(For any other time where the dependencies (folders in `/deps`) already exist)
```bash
./start-server
```

Note: Should be run in the root of the git repo.
Note: add `--init` only for the first run.

# Update server dependencies

`./update-repos.sh`
