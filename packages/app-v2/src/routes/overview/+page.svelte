<script lang="ts">
import { Chart } from "chart.js/auto";
import { onMount } from "svelte";

let pieChart: HTMLCanvasElement;
// Chart.register(PieController, ArcElement);

onMount(() => {
  const data = [
    { name: "Entertainment", value: 10 },
    { name: "Food", value: 20 },
    { name: "Cats", value: 20 },
    { name: "Others", value: 15 },
  ];

  new Chart(pieChart, {
    type: "pie",
    data: {
      labels: data.map((row) => row.name),
      datasets: [
        {
          label: "Category",
          data: data.map((row) => row.value),
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
            <td>3,432,321 VND</td>
            <td class="text-center">2,432,321 VND</td>
            <td class="text-right">1,987,321 VND</td>
        </tr>
        </tbody>
    </table>

    <a class="underline" href="/">Back</a>
</div>