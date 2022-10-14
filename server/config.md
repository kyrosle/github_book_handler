docker run -it --name postgres --restart always -e TZ='Asia/Shanghai' -e POSTGRES_PASSWORD='1234' -e ALLOW_IP_RANGE=0.0.0.0/0 -v /home/postgres/data:/var/lib/postgresql -p 55435:5432 -d postgres

