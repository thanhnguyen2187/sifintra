<script lang="ts">
type TransactionType = "income" | "expense";
type TransactionCategory =
  | "Entertainment"
  | "Others"
  | "Transportation"
  | "Cats";

interface Transaction {
  date: string;
  description: string;
  amount: number;
  type: TransactionType;
  category: TransactionCategory;
}

const records: Transaction[] = [
  {
    date: "2024-08-12 09:30",
    description: "Salary deposit",
    amount: 3500.0,
    type: "income",
    category: "Others",
  },
  {
    date: "2024-08-11 18:45",
    description: "Movie tickets",
    amount: 24.5,
    type: "expense",
    category: "Entertainment",
  },
  {
    date: "2024-08-11 14:20",
    description: "Cat food premium brand",
    amount: 45.99,
    type: "expense",
    category: "Cats",
  },
  {
    date: "2024-08-10 08:15",
    description: "Bus monthly pass",
    amount: 85.0,
    type: "expense",
    category: "Transportation",
  },
  {
    date: "2024-08-09 19:30",
    description: "Dinner at restaurant",
    amount: 67.8,
    type: "expense",
    category: "Entertainment",
  },
  {
    date: "2024-08-08 16:45",
    description: "Freelance project payment",
    amount: 750.0,
    type: "income",
    category: "Others",
  },
  {
    date: "2024-08-08 10:20",
    description: "Veterinarian checkup",
    amount: 120.0,
    type: "expense",
    category: "Cats",
  },
  {
    date: "2024-08-07 12:30",
    description: "Gas station",
    amount: 42.15,
    type: "expense",
    category: "Transportation",
  },
  {
    date: "2024-08-06 20:15",
    description: "Streaming service subscription",
    amount: 12.99,
    type: "expense",
    category: "Entertainment",
  },
  {
    date: "2024-08-05 11:45",
    description: "Cat toys and treats",
    amount: 28.75,
    type: "expense",
    category: "Cats",
  },
  {
    date: "2024-08-04 15:20",
    description: "Grocery shopping",
    amount: 89.32,
    type: "expense",
    category: "Others",
  },
  {
    date: "2024-08-03 09:10",
    description: "Uber ride to airport",
    amount: 35.6,
    type: "expense",
    category: "Transportation",
  },
  {
    date: "2024-08-02 21:45",
    description: "Concert tickets",
    amount: 125.0,
    type: "expense",
    category: "Entertainment",
  },
  {
    date: "2024-08-01 13:30",
    description: "Investment dividend",
    amount: 180.25,
    type: "income",
    category: "Others",
  },
  {
    date: "2024-07-31 17:55",
    description: "Cat litter refill",
    amount: 22.99,
    type: "expense",
    category: "Cats",
  },
];
</script>

<div class="flex flex-col gap-4">
    <span class="font-bold text-xl">Transactions</span>
    <div class="join">
        <button class="btn join-item">This week</button>
        <button class="btn join-item">This month</button>
        <button class="btn join-item">Last month</button>
    </div>
    <table class="table">
        <thead>
        <tr>
            <th>Date</th>
            <th>Description</th>
            <th>Amount</th>
            <th>Type</th>
            <th>Category</th>
        </tr>
        </thead>
        <tbody>
        {#each records as record}
            <tr>
                <td>{record.date}</td>
                <td>{record.description}</td>
                <td>{record.amount}</td>
                <td>{record.type}</td>
                <td>
                    <select class="select">
                        <option disabled selected>Pick a category</option>
                        <option>Crimson</option>
                        <option>Amber</option>
                        <option>Velvet</option>
                    </select>
                </td>
            </tr>
        {/each}
        </tbody>
        <tfoot>
        <tr>
            <td colspan="4">
                <div class="join">
                    <button class="join-item btn">1</button>
                    <button class="join-item btn">2</button>
                    <button class="join-item btn btn-disabled">...</button>
                    <button class="join-item btn">99</button>
                    <button class="join-item btn">100</button>
                </div>
            </td>
        </tr>
        </tfoot>
    </table>

    <a class="underline" href="/">Back</a>
</div>