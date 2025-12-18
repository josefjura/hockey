import { LitElement, PropertyValues } from 'lit';
export declare class CountrySelector extends LitElement {
    static formAssociated: boolean;
    static styles: import("lit").CSSResult;
    name: string;
    value: number | null;
    placeholder: string;
    iihfOnly: boolean;
    enabledOnly: boolean;
    private countries;
    private filteredCountries;
    private isOpen;
    private searchQuery;
    private loading;
    private selectedCountry;
    private internals;
    constructor();
    connectedCallback(): Promise<void>;
    updated(changedProperties: PropertyValues): void;
    private loadCountries;
    private toggleDropdown;
    private handleSearch;
    private selectCountry;
    private clearSelection;
    private handleClickOutside;
    private handleKeyDown;
    firstUpdated(): void;
    disconnectedCallback(): void;
    private getDropdownPosition;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'country-selector': CountrySelector;
    }
}
//# sourceMappingURL=country-selector.d.ts.map