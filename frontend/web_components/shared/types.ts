/**
 * Shared TypeScript types for web components
 */

import { TemplateResult } from 'lit';

/**
 * Column configuration for data tables
 */
export interface Column<T = any> {
  /** Property key to access in the data object */
  key: string;
  /** Column header label */
  label: string;
  /** Whether this column is sortable (default: true) */
  sortable?: boolean;
  /** Whether this column is filterable (default: false) */
  filterable?: boolean;
  /** Custom renderer function for cell content */
  renderer?: (value: any, row: T) => TemplateResult | string;
  /** CSS width for the column */
  width?: string;
  /** CSS text alignment */
  align?: 'left' | 'center' | 'right';
}

/**
 * Sort configuration
 */
export interface SortConfig {
  /** Column key to sort by */
  key: string;
  /** Sort order */
  order: 'asc' | 'desc';
}

/**
 * API error response
 */
export interface ApiError {
  message: string;
  status?: number;
}

/**
 * Generic API response wrapper
 */
export interface ApiResponse<T> {
  data?: T;
  error?: ApiError;
}

/**
 * Pagination info
 */
export interface PaginationInfo {
  currentPage: number;
  pageSize: number;
  totalItems: number;
  totalPages: number;
}

/**
 * Badge variant types
 */
export type BadgeVariant = 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'default';

/**
 * Size types for components
 */
export type ComponentSize = 'sm' | 'md' | 'lg';
