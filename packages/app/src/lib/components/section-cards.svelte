<script lang="ts">
import TrendingDownIcon from "@tabler/icons-svelte/icons/trending-down";
import TrendingUpIcon from "@tabler/icons-svelte/icons/trending-up";
import { Badge } from "$lib/components/ui/badge/index.js";
import * as Card from "$lib/components/ui/card/index.js";
</script>

<div
	class="
		*:data-[slot=card]:from-primary/5
		*:data-[slot=card]:to-card
		dark:*:data-[slot=card]:bg-card
		*:data-[slot=card]:shadow-xs
		@xl/main:grid-cols-2
		@5xl/main:grid-cols-4
		grid
		grid-cols-1
		gap-4
		px-4
		*:data-[slot=card]:bg-gradient-to-t
		lg:px-6"
>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Total Income</Card.Description>
			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
				$1,250.00
			</Card.Title>
		</Card.Header>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Total Expense</Card.Description>
			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
				$1,250.00
			</Card.Title>
		</Card.Header>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Current Balance</Card.Description>
			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
				$0
			</Card.Title>
		</Card.Header>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Growth Rate</Card.Description>
			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
				4.5%
			</Card.Title>
			<Card.Action>
				<Badge variant="outline">
					<TrendingUpIcon />
					+4.5%
				</Badge>
			</Card.Action>
		</Card.Header>
	</Card.Root>
</div>
