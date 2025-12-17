import { LitElement, PropertyValues } from 'lit';
export declare class CountrySelector extends LitElement {
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
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'country-selector': CountrySelector;
    }
}
//# sourceMappingURL=country-selector.d.ts.map