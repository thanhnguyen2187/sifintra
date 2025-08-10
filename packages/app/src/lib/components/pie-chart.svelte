<script lang="ts">
import { EditIcon } from "@lucide/svelte";
import { Arc, PieChart, Text } from "layerchart";
import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Chart from "$lib/components/ui/chart/index.js";
import * as Popover from "$lib/components/ui/popover/index.js";

const chartData = [
  { browser: "chrome", visitors: 275, color: "var(--color-chrome)" },
  { browser: "safari", visitors: 200, color: "var(--color-safari)" },
  { browser: "firefox", visitors: 187, color: "var(--color-firefox)" },
  { browser: "edge", visitors: 173, color: "var(--color-edge)" },
  { browser: "other", visitors: 90, color: "var(--color-other)" },
];

const chartConfig = {
  visitors: { label: "Visitors" },
  chrome: { label: "Chrome", color: "var(--chart-1)" },
  safari: { label: "Safari", color: "var(--chart-2)" },
  firefox: { label: "Firefox", color: "var(--chart-3)" },
  edge: { label: "Edge", color: "var(--chart-4)" },
  other: { label: "Other", color: "var(--chart-5)" },
} satisfies Chart.ChartConfig;
</script>

<Popover.Root>
    <Card.Root class="flex flex-col">
        <Card.Header class="items-center">
            <Card.Title>Expense by category</Card.Title>
        </Card.Header>
        <Card.Content class="flex-1">
            <Chart.Container
                config={chartConfig}
                class="mx-auto aspect-square max-h-[250px]"
            >
                <PieChart
                    data={chartData}
                    key="browser"
                    value="visitors"
                    cRange={chartData.map((d) => d.color)}
                    c="color"
                    props={{
                      pie: {
                        motion: "tween",
                      },
                    }}
                >
                    {#snippet tooltip()}
                        <Chart.Tooltip hideLabel />
                    {/snippet}
                    {#snippet arc({ props, visibleData, index })}
                        {@const browser = visibleData[index].browser}
                        <Arc {...props}>
                            {#snippet children({ getArcTextProps })}
                                <Text
                                    value={browser}
                                    {...getArcTextProps("centroid")}
                                    font-size="12"
                                    class="fill-background capitalize"
                                />
                            {/snippet}
                        </Arc>
                    {/snippet}
                </PieChart>
            </Chart.Container>
            <div>Add current categories here</div>
        </Card.Content>
        <Card.Footer class="flex-col gap-2 text-sm">
            <div class="flex items-center gap-2 font-medium leading-none">
                <Popover.Trigger>
                    <Button variant="secondary">
                        <EditIcon />
                        Edit Category
                    </Button>
                </Popover.Trigger>
            </div>
        </Card.Footer>
    </Card.Root>

    <Popover.Content
        interactOutsideBehavior="close"
    >
        table here
    </Popover.Content>
</Popover.Root>