<script lang="ts">
import { Chart } from "chart.js/auto";
import {
  endOfMonth,
  previousMonday,
  startOfMonth,
  startOfWeek,
  subMonths,
} from "date-fns";
import { onMount } from "svelte";
import { httpClient } from "$lib/default";

let pieChart: HTMLCanvasElement;
let chart: Chart | undefined;
let totalIncome = $state(0);
let totalExpense = $state(0);
let currentBalance = $state(0);
let fromTimestamp: number | undefined = $state(undefined);
let toTimestamp: number | undefined = $state(undefined);
// Chart.register(PieController, ArcElement);

$effect(() => {
  (async () => {
    const stats = await httpClient.fetchStats({ fromTimestamp, toTimestamp });

    const labels = stats.data.chartData.map((record) =>
      record.label === "_uncategorized" ? "Uncategorized" : record.label,
    );
    if (labels.length === 0) {
      labels.push("No expense");
    }
    const values = stats.data.chartData.map((record) => record.value);
    if (values.length === 0) {
      values.push(-1);
    }
    totalExpense = stats.data.totalExpenseVND;
    totalIncome = stats.data.totalIncomeVND;
    currentBalance = stats.data.currentBalanceVND;

    if (chart) {
      chart.destroy();
    }
    chart = new Chart(pieChart, {
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
  })();
});

function handleDateFilterButton(
  buttonType: "this-week" | "this-month" | "last-month" | "all-time",
) {
  const dateCurrent = new Date();
  switch (buttonType) {
    case "this-week": {
      const dateWeekStart = startOfWeek(dateCurrent);
      fromTimestamp = dateWeekStart.getTime() / 1_000;
      toTimestamp = dateCurrent.getTime() / 1_000;
      break;
    }
    case "this-month": {
      const dateMonthStart = startOfMonth(dateCurrent);
      fromTimestamp = dateMonthStart.getTime() / 1_000;
      toTimestamp = dateCurrent.getTime() / 1_000;
      break;
    }
    case "last-month": {
      const date1MonthAgo = subMonths(dateCurrent, 1);
      const dateMonthStart = startOfMonth(date1MonthAgo);
      const dateMonthEnd = endOfMonth(date1MonthAgo);
      fromTimestamp = dateMonthStart.getTime() / 1_000;
      toTimestamp = dateMonthEnd.getTime() / 1_000;
      break;
    }
    case "all-time": {
      fromTimestamp = undefined;
      toTimestamp = undefined;
      break;
    }
  }
}
</script>

<div class="flex flex-col gap-4">
    <span class="font-bold text-xl">Overview</span>
    <div class="join">
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("this-week")}
        >
            This week
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("this-month")}
        >
            This month
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("last-month")}
        >
            Last month
        </button>
        <button
            class="btn join-item"
            onclick={() => handleDateFilterButton("all-time")}
        >
            All time
        </button>
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