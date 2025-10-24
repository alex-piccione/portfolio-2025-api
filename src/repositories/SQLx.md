# SQLx

SQLx-CLI needs to be installed: ``cargo install sqlx-cli --no-default-features --features "postgres"``

SQLx use the **DATABASE_URL** environment variable to read the connection string.  
There is an _.env_ file in the root folder, so SQLx commands **MUST be run from the root folder**  
(or use this: `export DATABASE_URL="postgresql://username:password@localhost/database_name"`).  
  
The application, at start, will check and execute the migrations (inside _/migrations_ folder) if the configuration enabled it.  
  
These commands can be used to manage database creation and changes:
- ``cargo sqlx prepare``         # Generate query metadata (local cache used by static analizer)
- ``cargo sqlx migrate run``     # Run database migrations
- ``cargo sqlx database create`` # Create database
- ``cargo sqlx database drop``   # Drop database

Refer to the *sqlx_commands.sh* file that has prepared commands.  


```sql
select * from _sqlx_migrations  [TBC]
```


## Add a new entity

1. Create the entitty in /entities
2. ``cargo sqlx migrate add create_<entity>_table``
   it will create an empty file in /migrations folder
3. create a valid SQL command using a real database,
   once done, delete the table and update the create_<entity>_table
4. ``cargo sqlx migrate run`` will execute the migration


## Issues

### Failed to apply database migrations: migration 20250901085448 was previously applied but has been modified

It is caused by a change on the migration file.  
Check the git history of that file, and try to recover the initial "execute" version.  
`file=$(git ls-files | grep 20250901085448) ; echo $file`  
  
If there is only one commit use this: `commit=$(git log --follow --format="%H" $file) ; echo $commit`  
otherwise just sabve the commit: `commit=...`  
  
Finally get the content:
```sh
# Just print it
git show $commit:$file

# Save it
git show $commit:$file > file_at_commit.txt
```

Can be just a CRLF vs CR different file format (Windows vs Linux).  
``filr $file`` will tell you if hte file uses CRLF (it should not).  
See the README file in the _migrations_ folder.

### error communicating with database: An established connection was aborted by the software in your host machine.

> error communicating with database: An established connection was aborted by the software in your host machine. (os error 10053)rust-analyzermacro-error

Check what connection string is defined in _/.env_.  
If it is localhost it probably point to a local Docker container, check itf it is running.  

``cargo build`` has to be successful and restart VS Code will sometime solve the problem.  