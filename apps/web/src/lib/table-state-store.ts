import { Store } from "@tanstack/store"
import type { PaginationState, SortingState } from "@tanstack/react-table"

const DEFAULT_PAGE_SIZE = 20

export type TTableState = {
	sorting: SortingState
	pagination: PaginationState
}

export const createTableStore = (): Store<TTableState> =>
	new Store<TTableState>({
		sorting: [],
		pagination: { pageIndex: 0, pageSize: DEFAULT_PAGE_SIZE },
	})

export const applySorting = (
	store: Store<TTableState>,
	next: SortingState | ((current: SortingState) => SortingState),
): void =>
	store.setState((state) => ({
		...state,
		sorting: typeof next === "function" ? next(state.sorting) : next,
	}))

export const applyPagination = (
	store: Store<TTableState>,
	next: PaginationState | ((current: PaginationState) => PaginationState),
): void =>
	store.setState((state) => ({
		...state,
		pagination: typeof next === "function" ? next(state.pagination) : next,
	}))
