<script lang="ts">
import { EditIcon, PlusIcon, TrashIcon } from "@lucide/svelte";
import { quantize } from "d3-interpolate";
import { interpolateRainbow } from "d3-scale-chromatic";
import { Arc, PieChart, Text } from "layerchart";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Chart from "$lib/components/ui/chart/index.js";
import * as Table from "$lib/components/ui/table/index.js";

const records = [{ id: crypto.randomUUID(), name: "John", value: 100 }];

const chartData = [
  { browser: "chrome", visitors: 275, color: "var(--color-chrome)" },
  { browser: "safari", visitors: 200, color: "var(--color-safari)" },
  { browser: "firefox", visitors: 187, color: "var(--color-firefox)" },
  { browser: "edge", visitors: 173, color: "var(--color-edge)" },
  { browser: "other", visitors: 90, color: "var(--color-other)" },
];
</script>

<Card.Root class="flex flex-col">
    <Card.Header class="items-center text-center">
        <Card.Title>Expense by category</Card.Title>
    </Card.Header>
    <Card.Content class="flex-1">
        <Chart.Container
            class="mx-auto aspect-square max-h-[250px]"
        >
            <PieChart
                data={chartData}
                key="browser"
                value="visitors"
                props={{
                  pie: {
                    motion: "tween",
                  },
                }}
                cRange={quantize(interpolateRainbow, chartData.length + 1)}
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
        <div class="mt-6">
            <Table.Root>
                <Table.Header>
                    <Table.Row>
                        <Table.Head>Name</Table.Head>
                        <Table.Head>Amount</Table.Head>
                        <Table.Head class="text-right">Actions</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each chartData as item, index}
                        <Table.Row>
                            <Table.Cell>{item.browser}</Table.Cell>
                            <Table.Cell>{item.visitors}</Table.Cell>
                            <Table.Cell class="text-right">
                                <Button variant="secondary" size="icon">
                                    <EditIcon />
                                </Button>
                                <Button variant="secondary" size="icon">
                                    <TrashIcon />
                                </Button>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>
    </Card.Content>
    <Card.Footer class="flex-col gap-2 text-sm">
        <div class="flex items-center gap-2 font-medium leading-none">
            <Button variant="secondary">
                <PlusIcon />
                Add category
            </Button>
        </div>
    </Card.Footer>
</Card.Root>
