# Kronos

A small Discord Bot

## Environment variables

| Variable        | Description                                                                          | Required?          |
|:----------------|:-------------------------------------------------------------------------------------|:-------------------|
| BOT_TOKEN       | Discord Bot Token                                                                    | :heavy_check_mark: |
| VERBOSITY       | Verbosity of console output. Possible values are `debug`, `info`, `warn` or `error`. | :heavy_check_mark: |
|                 |                                                                                      |                    |
| MONGO_USER      | Username to use to access the db.                                                    | :heavy_check_mark: |
| MONGO_PASSWORD  | Password of the user for the db.                                                     | :heavy_check_mark: |
| MONGO_CLUSTER   | Name of the cluster.                                                                 | :heavy_check_mark: |
| DATABASE_NAME   | Name of the db that will be created in the cluster.                                  | :heavy_check_mark: |
| LOG_DATE_FORMAT | Overwrite the date format in the logs.                                               |                    |
