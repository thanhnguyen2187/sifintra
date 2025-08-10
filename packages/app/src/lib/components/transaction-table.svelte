<script lang="ts">
import { Badge } from "$lib/components/ui/badge";
import * as Card from "$lib/components/ui/card/index.js";
import * as Pagination from "$lib/components/ui/pagination/index.js";
import * as Select from "$lib/components/ui/select/index.js";
import * as Table from "$lib/components/ui/table/index.js";

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
                            <Select.Root type="single">
                                <Select.Trigger>Pick</Select.Trigger>
                                <Select.Content>
                                    <Select.Item value="">Category 1</Select.Item>
                                    <Select.Item value="">Category 2</Select.Item>
                                    <Select.Item value="">Category 3</Select.Item>
                                </Select.Content>
                            </Select.Root>
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer>
        <Pagination.Root count={100} perPage={10}>
            {#snippet children({ pages, currentPage })}
                <Pagination.Content>
                    <Pagination.Item>
                        <Pagination.PrevButton />
                    </Pagination.Item>
                    {#each pages as page (page.key)}
                        {#if page.type === "ellipsis"}
                            <Pagination.Item>
                                <Pagination.Ellipsis />
                            </Pagination.Item>
                        {:else}
                            <Pagination.Item>
                                <Pagination.Link {page} isActive={currentPage === page.value}>
                                    {page.value}
                                </Pagination.Link>
                            </Pagination.Item>
                        {/if}
                    {/each}
                    <Pagination.Item>
                        <Pagination.NextButton />
                    </Pagination.Item>
                </Pagination.Content>
            {/snippet}
        </Pagination.Root>
    </Card.Footer>
</Card.Root>
