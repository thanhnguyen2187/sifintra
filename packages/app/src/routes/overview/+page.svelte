<script lang="ts">
import { Chart } from "chart.js/auto";
import { onMount } from "svelte";
import { httpClient } from "$lib/default";

let pieChart: HTMLCanvasElement;
let totalIncome = $state(0);
let totalExpense = $state(0);
let currentBalance = $state(0);
// Chart.register(PieController, ArcElement);

onMount(async () => {
  const stats = await httpClient.fetchStats();

  const labels = stats.data.chartData.map((record) =>
    record.label === "_uncategorized" ? "Uncategorized" : record.label,
  );
  const values = stats.data.chartData.map((record) => record.value);
  totalExpense = stats.data.totalExpenseVND;
  totalIncome = stats.data.totalIncomeVND;
  currentBalance = stats.data.currentBalanceVND;

  new Chart(pieChart, {
    type: "pie",
    data: {
      labels,
      datasets: [
        {
          data: values,
        },
      ],
    },
    options: {
      plugins: {
        legend: {
          position: "bottom",
          labels: {
            padding: 40,
          },
        },
      },
    },
  });
});
</script>

<div class="flex flex-col gap-4">
    <span class="font-bold text-xl">Overview</span>
    <div class="join">
        <button class="btn join-item">This week</button>
        <button class="btn join-item">This month</button>
        <button class="btn join-item">Last month</button>
    </div>
    <canvas
        bind:this={pieChart}
        class="max-h-[40em] pt-2"
    ></canvas>
    <table class="table">
        <thead>
        <tr>
            <th>Total Income</th>
            <th class="text-center">Total Expense</th>
            <th class="text-right">Current Balance</th>
        </tr>
        </thead>
        <tbody>
        <tr>
            <td>{totalIncome} VND</td>
            <td class="text-center">{totalExpense} VND</td>
            <td class="text-right">{currentBalance} VND</td>
        </tr>
        </tbody>
    </table>

    <a class="underline" href="/">Back</a>
</div>