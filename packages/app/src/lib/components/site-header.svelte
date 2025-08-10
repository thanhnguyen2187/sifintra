<script lang="ts">
import {
  CalendarDate,
  DateFormatter,
  type DateValue,
  getLocalTimeZone,
} from "@internationalized/date";
import { GithubIcon } from "@lucide/svelte";
import CalendarIcon from "@lucide/svelte/icons/calendar";
import type { DateRange } from "bits-ui";
import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
import * as Popover from "$lib/components/ui/popover/index.js";
import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
import * as Select from "$lib/components/ui/select/index.js";
import { Separator } from "$lib/components/ui/separator/index.js";
import * as Sidebar from "$lib/components/ui/sidebar/index.js";
import * as Tabs from "$lib/components/ui/tabs/index.js";
import { cn } from "$lib/utils.js";

const df = new DateFormatter("en-US", {
  dateStyle: "medium",
});

let value: DateRange = $state({
  start: new CalendarDate(2022, 1, 20),
  end: new CalendarDate(2022, 1, 20).add({ days: 20 }),
});

let startValue: DateValue | undefined = $state(undefined);

const items = [
  { value: 0, label: "Today" },
  { value: 7, label: "This week" },
  { value: 30, label: "This month" },
  { value: 31, label: "Last month" },
];
</script>

<header
    class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear"
>
    <div class="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
        <h1 class="text-base font-medium py-4">
            <b>
                Simple Financial Tracker
            </b>
        </h1>
        <Separator orientation="vertical" />
        <Popover.Root>
            <Popover.Trigger
                class={cn(
					buttonVariants({ variant: "outline" }),
					!value && "text-muted-foreground"
				)}
            >
                <CalendarIcon class="mr-2 size-4" />
                {#if value && value.start}
                    {#if value.end}
                        {df.format(value.start.toDate(getLocalTimeZone()))} - {df.format(
                        value.end.toDate(getLocalTimeZone())
                    )}
                    {:else}
                        {df.format(value.start.toDate(getLocalTimeZone()))}
                    {/if}
                {:else if startValue}
                    {df.format(startValue.toDate(getLocalTimeZone()))}
                {:else}
                    Pick a date range
                {/if}
            </Popover.Trigger>
            <Popover.Content class="flex w-auto flex-col space-y-2 p-2" align="start">
                <RangeCalendar
                    bind:value
                    onStartValueChange={(v) => {
                        startValue = v;
                    }}
                    numberOfMonths={1}
                />
                <div class="flex flex-row-reverse mb-4 mr-3.5">
                    <Select.Root
                        type="single"
                        name="picked"
                    >
                        <Select.Trigger>
                            Quick pick
                        </Select.Trigger>
                        <Select.Content>
                            {#each items as item (item.value)}
                                <Select.Item
                                    value={`${item.value}`}
                                >
                                    {item.label}
                                </Select.Item>
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            </Popover.Content>
        </Popover.Root>
        <div class="ml-auto flex items-center gap-2">
            <Button
                href="https://github.com/shadcn-ui/ui/tree/main/apps/v4/app/(examples)/dashboard"
                variant="ghost"
                size="sm"
                class="dark:text-foreground hidden sm:flex"
                target="_blank"
                rel="noopener noreferrer"
            >
                <GithubIcon />
                GitHub
            </Button>

        </div>
    </div>
</header>
