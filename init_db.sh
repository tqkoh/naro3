wget https://downloads.mysql.com/docs/world-db.zip
unzip world-db.zip
rm world-db.zip
sed -i -e '/^CREATE DATABASE/d' world-db/world.sql
sed -i -e "s/USE \`world\`\;/USE \`$MARIADB_DATABASE\`\;/g" world-db/world.sql
mysql -u $MARIADB_USERNAME -p$MARIADB_PASSWORD < world-db/world.sql
