* Memcached Protocol
  - There are 2 kinds of lines text (commands), unstructured data.
  - All lines ends with /r/n.
  - Keys are 250 characters. Keys must not include control characters or whitespace.
* Commands
  There are 3 kinds of commands.
  - Storage commands (there are six: "set", "add", "replace", "append", "prepend" and "cas")
  - Retrieval commands ("get", "gets", "gat", and "gats")
  - All other commands don't involve unstructured data.
  - Command names are lower-case and are case-sensitive.

** Meta commands experimental.
   <cm> <key> <flag1> <flag2> <...>\r\n
*** examples
    - GET : mg foo t f v
    - GETS: mg foo t f c v
    - TOUCH: mg foo T30

    Some example flag values:
    - l flag will show the number of seconds since the item was last accessed.
    - h flag will return 0 or 1 based on if the item has ever been fetched since being stored.
    - t flag will return the number of seconds remaining until the item expires (-1 for infinite)

** Expiration times:
   TTL can be:
   - Number of seconds since January 1, 1970, as a 32-bit value.
   - A number of seconds starting from current time

   Expiration time roughly +/- 1s

** Error responses:
   - "ERROR\r\n":  means the client sent a nonexistent command name.
   - "CLIENT_ERROR <error>\r\n": the input doesn't conform to the protocol in some way
   - "SERVER_ERROR <error>\r\n".


** Authentication:
   Optional username/password token authentication (see -Y option). Used by sending a fake "set" command with any key:

   set <key> <flags> <exptime> <bytes>\r\n
   username password\r\n

   responses:
   - "STORED\r\n" indicates success.
   - "CLIENT_ERROR [message]\r\n" will be returned if authentication fails for any reason.

* Storage Commands:
** First command is sent:
  <command name> <key> <flags> <exptime> <bytes> [noreply]\r\n

  - set: store this data.

  - add: store this data, but only if the server *doesn't* already hold data for this key.
  - replace: store this data, but only if the server *does* already hold data for this key.

  - append: add this data to an existing key after existing data.

  - prepend" means "add this data to an existing key before existing data".

*** Arguments
  <flags>: An arbitrary 16-bit unsigned integer (written out in
  decimal) that the server stores along with the data and sends back
  when the item is retrieved.
  <bytes>: the number of bytes in the data block to follow, *not*
  including the delimiting \r\n.


** After this line, the client sends the data block:

<data block>\r\n

- <data block> is a chunk of arbitrary 8-bit data of length <bytes>
  from the previous line.

** Responses
   - "STORED\r\n"
   - "NOT_STORED\r\n"
   - "EXISTS\r\n"
   - "NOT_FOUND\r\n"

* Retrieval commands:
- get <key>*\r\n
- gets <key>*\r\n

After this command, the client expects zero or more items, each of
which is received as a text line followed by a data block. After all
the items have been transmitted, the server sends the string

"END\r\n"

* Deletion:
- delete <key> [noreply]\r\n

** Responses:
   - "DELETED\r\n"
   - "NOT_FOUND\r\n"

* Increment/Decrement
  Value should be 64-bit unsigned integer
  - incr <key> <value> [noreply]\r\n
  - decr <key> <value> [noreply]\r\n
** Responses
   - "NOT_FOUND\r\n"
   - <value>\r\n
* Touch
   - touch <key> <exptime> [noreply]\r\n

** Responses
   - "TOUCHED\r\n"
   - "NOT_FOUND\r\n"

* Get And Touch
   - gat <exptime> <key>*\r\n
   - gats <exptime> <key>*\r\n


* Meta Debug
   - me <key>\r\n
** Responses
   - ME <key> <k>=<v>*\r\n


     exp   = expiration time
     la    = time in seconds since last access
     cas   = CAS ID
     fetch = whether an item has been fetched before
     cls   = slab class id
     size  = total size in bytes

   - EN\r\n


* Meta Get
  this can replace all of the commands: "get", "gets", "gat", "gats", "touch"
  - mg <key> <flags>*\r\n


* .....

* Statistics
- stats\r\n

* Other commands
- flush_all
- cache_memlimit
- shutdown
- version
- quit
- misbehave
