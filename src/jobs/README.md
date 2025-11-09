# Jobs

## cron

Standard 5 fields syntax.  
Quarx 6 fields syntax.  
  
| cron         | result               | explamnation                                                                                  |
| ---          | ---                  | ---                                                                                           |
| */10 * * * * | 10 minutes           | At every 10th minute, every hour, every day, every month, every day of the week.              |
| */30 * * * * | 30 minutes           | At every 30th minute of every hour, every day, every month, every day of the week.            |
| 0 * * * *    | 1 hour               | At minute 0 of every hour, every day, every month, every day of the week.                     |
| 0 */2 * * *  | 2 hours              | At minute 0 of every 2nd hour, every day, every month, every day of the week.                 |
| 0 8 * * *    | daily, at 8 AM       | At minute 0, hour 8, every day, every month, every day of the week.                           |


## Coingecko

DOCS: https://docs.coingecko.com  
DEMO API URL: https://api.coingecko.com/api/v3/


