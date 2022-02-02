
# Server requirements (as far as I know):

key: `$ needs-to-be-on-PATH-program`

 * `$ cargo` version >= 1.53
 * `$ git` version >= 2.20 (I also use my personal SSH profiles to clone which should have configuration but currently don't)
 * `$ bash` (duh)
 * `rustc` version >= 1.53

# Running the server

Easiest way to run the server is by running

(For the first time)
```bash
bash start-server.sh . <host:port> --init
```

(For any other time where the dependencies (folders in `/deps`) already exist)
```bash
bash start-server.sh . <host:port>
```

Note: Should be run in the root of the git repo.
Note: add `--init` only for the first run.

To run with a website other than my personal website, go see [my rust webserver project](https://github.com/veggiebob/rust-webserver.git) (which this is based on) lol

# Update server dependencies

You can (obviously) "update" the dependencies by killing the server and re-running the server script.

Or use `bash update-repos.sh` to pull website content and recompile and pull the parse demo.