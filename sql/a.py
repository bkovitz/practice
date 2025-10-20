import psycopg

def run(q):
    #with psycopg.connect("postgresql://localhost:5432/practice") as conn:
    print()
    with psycopg.connect("dbname=practice") as conn:
        cur = conn.execute(q)
        for row in cur:
            print(row)

#run('select now()')
#run('select * from employee')

run('''
SELECT tablename
FROM pg_tables
WHERE schemaname = 'public'
''')

"""
run('''
SELECT *
FROM customer
''')
"""

# Each customer's total spending
"""
run('''
SELECT c.*, SUM(o.amount)
FROM customers c
JOIN orders o on c.id = o.customer_id
GROUP BY c.id
ORDER BY c.name
''')
"""

# Customer who spent the most
run('''
SELECT c.*, SUM(o.amount) as su
FROM customers c
JOIN orders o ON c.id = o.customer_id
GROUP BY c.id
ORDER BY su DESC
LIMIT 1
''')
