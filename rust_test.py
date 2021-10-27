import requests, time

if __name__ == "__main__":
    users = 1000
    users_per_sec_goal = 100
    url = "http://localhost:8080/api/data"

    start = time.time()

    for i in range(users):
        requests.get(url)

    time_elapsed = time.time() - start
    print(f"\n\nIt took {time_elapsed / (users / users_per_sec_goal)} seconds to serve the data to {users_per_sec_goal} users (simulated {users} users).")
    print(f"This gives a bandwidth of {users // time_elapsed} users per second")
