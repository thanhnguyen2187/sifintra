<script lang="ts">
import { Badge } from "$lib/components/ui/badge";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Pagination from "$lib/components/ui/pagination/index.js";
import * as Select from "$lib/components/ui/select/index.js";
import * as Table from "$lib/components/ui/table/index.js";

let value: string = $state("");

const invoices = [
  {
    invoice: "2022-01-01 14:22",
    paymentStatus: "Paid",
    totalAmount: "$250.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "2022-01-01 15:22",
    paymentStatus: "Pending",
    totalAmount: "$150.00",
    paymentMethod: "PayPal",
  },
  {
    invoice: "2022-01-01 16:21",
    paymentStatus: "Unpaid",
    totalAmount: "$350.00",
    paymentMethod: "Bank Transfer",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Paid",
    totalAmount: "$450.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Paid",
    totalAmount: "$550.00",
    paymentMethod: "PayPal",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Pending",
    totalAmount: "$200.00",
    paymentMethod: "Bank Transfer",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Unpaid",
    totalAmount: "$300.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Unpaid",
    totalAmount: "$300.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Unpaid",
    totalAmount: "$300.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "2022-01-01 16:22",
    paymentStatus: "Unpaid",
    totalAmount: "$300.00",
    paymentMethod: "Credit Card",
  },
];
</script>

<Card.Root>
    <Card.Content>
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head class="w-[100px]">Date</Table.Head>
                    <Table.Head>Description</Table.Head>
                    <Table.Head class="text-right">Amount</Table.Head>
                    <Table.Head>Type</Table.Head>
                    <Table.Head>Category</Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each invoices as invoice, index }
                    <Table.Row>
                        <Table.Cell class="font-medium">{invoice.invoice}</Table.Cell>
                        <Table.Cell>{invoice.paymentStatus}</Table.Cell>
                        <Table.Cell class="text-right">{invoice.totalAmount}</Table.Cell>
                        <Table.Cell>
                            {#if index % 2 === 0}
                                <Badge
                                    variant="outline"
                                    class="bg-red-700"
                                >
                                    expense
                                </Badge>
                            {:else}
                                <Badge
                                    variant="outline"
                                    class="bg-green-700"
                                >
                                    income
                                </Badge>
                            {/if}
                        </Table.Cell>
                        <Table.Cell>
                            <DropdownMenu.Root>
                                <DropdownMenu.Trigger>
                                    {#snippet child({ props })}
                                        <Button {...props} variant="outline">
                                            Entertainment
                                        </Button>
                                    {/snippet}
                                </DropdownMenu.Trigger>
                                <DropdownMenu.Content class="w-56">
                                    <DropdownMenu.Group>
                                        <DropdownMenu.Label>Panel Position</DropdownMenu.Label>
                                        <DropdownMenu.Separator />
                                        <DropdownMenu.RadioGroup bind:value={value}>
                                            <DropdownMenu.RadioItem value="top">Top</DropdownMenu.RadioItem>
                                            <DropdownMenu.RadioItem value="bottom">Bottom</DropdownMenu.RadioItem>
                                            <DropdownMenu.RadioItem value="right">Right</DropdownMenu.RadioItem>
                                        </DropdownMenu.RadioGroup>
                                    </DropdownMenu.Group>
                                </DropdownMenu.Content>
                            </DropdownMenu.Root>
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer class="flex-1 flex-col">
        <div class="flex-1"></div>
        <div>Total x items</div>
    </Card.Footer>
</Card.Root>
