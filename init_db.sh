wget https://downloads.mysql.com/docs/world-db.zip
unzip world-db.zip
rm world-db.zip
sed -i -e '/^CREATE DATABASE/d' world-db/world.sql
sed -i -e 's/^DROP TABLE IF EXISTS `city`;/IF NOT EXISTS `city` THEN/g' world-db/world.sql
sed -i -e 's/^DROP TABLE IF EXISTS `country`;/IF NOT EXISTS `country` THEN/g' world-db/world.sql
sed -i -e 's/^DROP TABLE IF EXISTS `countrylanguage`;/IF NOT EXISTS `countrylanguage` THEN/g' world-db/world.sql
sed -i -e sZ'/\*!40101 SET character_set_client = @saved_cs_client \*/;'Z'END IF'Zg world-db/world.sql
sed -i -e "s/USE \`world\`\;/USE \`$MARIADB_DATABASE\`\;/g" world-db/world.sql
# mysql -u $MARIADB_USERNAME -p$MARIADB_PASSWORD < world-db/world.sql
