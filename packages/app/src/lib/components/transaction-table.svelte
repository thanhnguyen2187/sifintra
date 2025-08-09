<script lang="ts">
import * as Card from "$lib/components/ui/card/index.js";
import * as Select from "$lib/components/ui/select/index.js";
import * as Table from "$lib/components/ui/table/index.js";

const invoices = [
  {
    invoice: "INV001",
    paymentStatus: "Paid",
    totalAmount: "$250.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "INV002",
    paymentStatus: "Pending",
    totalAmount: "$150.00",
    paymentMethod: "PayPal",
  },
  {
    invoice: "INV003",
    paymentStatus: "Unpaid",
    totalAmount: "$350.00",
    paymentMethod: "Bank Transfer",
  },
  {
    invoice: "INV004",
    paymentStatus: "Paid",
    totalAmount: "$450.00",
    paymentMethod: "Credit Card",
  },
  {
    invoice: "INV005",
    paymentStatus: "Paid",
    totalAmount: "$550.00",
    paymentMethod: "PayPal",
  },
  {
    invoice: "INV006",
    paymentStatus: "Pending",
    totalAmount: "$200.00",
    paymentMethod: "Bank Transfer",
  },
  {
    invoice: "INV007",
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
                {#each invoices as invoice (invoice)}
                    <Table.Row>
                        <Table.Cell class="font-medium">{invoice.invoice}</Table.Cell>
                        <Table.Cell>{invoice.paymentStatus}</Table.Cell>
                        <Table.Cell class="text-right">{invoice.totalAmount}</Table.Cell>
                        <Table.Cell>expense</Table.Cell>
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
</Card.Root>
