<script lang="ts">
import { CheckIcon, EditIcon, PlusIcon, TrashIcon } from "@lucide/svelte";
import { quantize } from "d3-interpolate";
import { interpolateRainbow } from "d3-scale-chromatic";
import { Arc, PieChart, Text } from "layerchart";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Chart from "$lib/components/ui/chart/index.js";
import { Input } from "$lib/components/ui/input/index.js";
import * as Table from "$lib/components/ui/table/index.js";

const chartData = [
  { id: crypto.randomUUID(), value: 275, name: "Entertainment" },
  { id: crypto.randomUUID(), value: 200, name: "Food" },
  { id: crypto.randomUUID(), value: 187, name: "Transportation" },
  { id: crypto.randomUUID(), value: 173, name: "Shopping" },
  { id: crypto.randomUUID(), value: 90, name: "Others" },
];
</script>

<Card.Root class="flex flex-col">
    <Card.Header class="items-center text-center">
        <Card.Title>Expense by category</Card.Title>
    </Card.Header>
    <Card.Content class="flex-1">
        <Chart.Container
            class="mx-auto aspect-square"
        >
            <PieChart
                data={chartData}
                key="id"
                value="value"
                props={{
                  pie: {
                    motion: "tween",
                  },
                }}
                cRange={quantize(interpolateRainbow, chartData.length + 1)}
            >
                {#snippet tooltip()}
                    <div></div>
                {/snippet}
                {#snippet arc({ props, visibleData, index })}
                    {@const name = visibleData[index].name}
                    <Arc {...props}>
                        {#snippet children({ getArcTextProps })}
                            <Text
                                value={name}
                                {...getArcTextProps("centroid")}
                                font-size="12"
                                class="fill-background"
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
                    {#each chartData as item}
                        <Table.Row>
                            <Table.Cell>{item.name}</Table.Cell>
                            <Table.Cell>{item.value}</Table.Cell>
                            <Table.Cell class="text-right">
                                <Button variant="outline" size="icon">
                                    <EditIcon />
                                </Button>
                                <Button variant="outline" size="icon">
                                    <TrashIcon />
                                </Button>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                    <Table.Row>
                        <Table.Cell colspan={2}>
                            <Input type="text" />
                        </Table.Cell>
                        <Table.Cell class="text-right">
                            <Button variant="outline" size="icon">
                                <CheckIcon />
                            </Button>
                            <Button variant="outline" size="icon">
                                <TrashIcon />
                            </Button>
                        </Table.Cell>
                    </Table.Row>
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
