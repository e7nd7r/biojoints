
## Setting up database

### Creating a user in MySql 

```sh
CREATE USER 'user'@'%' IDENTIFIED BY 'password';

GRANT ALL PRIVILEGES ON *.* TO '<user>'
```

### How to load database

1. Download mysql shell for MacOs

https://dev.mysql.com/downloads/shell/

2. Once installed run the following commands.

```sh
util.loadDump("/Users/esteban/Desktop/linbos_backup/linbos_catalog_backup", {dryRun: true})
util.loadDump("/Users/esteban/Desktop/linbos_backup/linbos_catalog_backup", {progressFile :"/Users/esteban/backuplog.json",threads:4,backgroundThreads:4,maxBytesPerTransaction:"4096"})
util.loadDump("/Users/esteban/Desktop/linbos_backup/cerambicidos_cms_backup", {progressFile :"/Users/esteban/
cerambicidos_cms_backuplog.json",threads:4,backgroundThreads:4,maxBytesPerTransaction:"4096"})
```

### Enabling LOAD DATA LOCAL INFILE

```sh
SHOW GLOBAL VARIABLES LIKE 'local_infile';
SET GLOBAL local_infile = 'ON';
SHOW GLOBAL VARIABLES LIKE 'local_infile';
```

### Enabling remote access

1. Open mysql config

```sh
sudo nano /etc/mysql/mysql.conf.d/mysqld.cnf
```

2. Set `bind-address` value to `0.0.0.0`

3. Restart mysql

```sh
sudo systemctl restart mysql
```

# query examples

MATCH (f:Family {family: 'Cerambycidae'})
MATCH path = (f)-[:BELONGS_TO*0..]->(n)
RETURN n

MATCH (g:Genus {family: 'Cerambycidae'})
MATCH path = (g)-[:BELONGS_TO*0..]->(n)
RETURN n

MATCH (s:Specie)-[:BELONGS_TO]->(:Genus)
MATCH path = (s)-[:BELONGS_TO*0..]->(n)
RETURN n


MATCH (s:Specie)-[:BELONGS_TO]->(:Genus {family: "Cerambycidae"})
MATCH path = (s)-[:BELONGS_TO*0..]->(n)
RETURN n

MATCH (s:Specie)-[:BELONGS_TO]->(g:Genus)-[:BELONGS_TO]->(f:Family {name: "Cerambycidae"})
RETURN DISTINCT g

MATCH (Specie)-[r:BELONGS_TO]->(Genus) DELETE r

MATCH (s:Specie) DELETE s

MATCH (n) RETURN n

MATCH (s:Specie)-[:LOCATED_AT]->(st:State {code: "dur"}) RETURN s

MATCH (s:Specie)-[:LOCATED_IN]->(st:State {code: "oax"}) 
MATCH path = (s)-[:BELONGS_TO*0..]->(n)
RETURN n, st

MATCH (s:Specie)-[:LOCATED_IN]->(st:State {code: "col"}) 
MATCH path = (s)-[:BELONGS_TO*0..]->(n)
RETURN n, st

## Roadmap

[ ] Migrate taxonomies.
[ ] Configuration files.
[ ] App builder and endpoint creators.
[ ] CRUD endpoints.
