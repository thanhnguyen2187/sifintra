<script lang="ts">
import { Chart } from "chart.js/auto";
import { onMount } from "svelte";

let pieChart: HTMLCanvasElement;
// Chart.register(PieController, ArcElement);

onMount(() => {
  const data = [
    { year: 2010, count: 10 },
    { year: 2011, count: 20 },
    { year: 2012, count: 15 },
    { year: 2013, count: 25 },
    { year: 2014, count: 22 },
    { year: 2015, count: 30 },
    { year: 2016, count: 28 },
  ];

  new Chart(pieChart, {
    type: "pie",
    data: {
      labels: data.map((row) => row.year),
      datasets: [
        {
          label: "Category",
          data: data.map((row) => row.count),
        },
      ],
    },
    options: {
      plugins: {
        title: {
          position: "bottom",
        },
      },
    },
  });
});
</script>

<div class="flex flex-col gap-4">
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

    <canvas
        bind:this={pieChart}
        class="max-h-[40em]"
    ></canvas>
</div>