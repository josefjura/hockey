var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
let CountrySelector = class CountrySelector extends LitElement {
    constructor() {
        super(...arguments);
        this.name = '';
        this.value = null;
        this.placeholder = 'Select a country';
        this.iihfOnly = false;
        this.enabledOnly = false;
        this.countries = [];
        this.filteredCountries = [];
        this.isOpen = false;
        this.searchQuery = '';
        this.loading = false;
        this.selectedCountry = null;
        this.handleClickOutside = (e) => {
            if (!this.contains(e.target)) {
                this.isOpen = false;
            }
        };
    }
    async connectedCallback() {
        super.connectedCallback();
        await this.loadCountries();
    }
    updated(changedProperties) {
        if (changedProperties.has('value') && this.value !== null) {
            this.selectedCountry = this.countries.find(c => c.id === this.value) || null;
        }
    }
    async loadCountries() {
        this.loading = true;
        try {
            const params = new URLSearchParams();
            if (this.iihfOnly)
                params.set('iihf_only', 'true');
            if (this.enabledOnly)
                params.set('enabled_only', 'true');
            const response = await fetch(`/api/countries?${params}`);
            if (!response.ok)
                throw new Error('Failed to fetch countries');
            this.countries = await response.json();
            this.filteredCountries = this.countries;
            // Set selected country if value is provided
            if (this.value !== null) {
                this.selectedCountry = this.countries.find(c => c.id === this.value) || null;
            }
        }
        catch (error) {
            console.error('Error loading countries:', error);
        }
        finally {
            this.loading = false;
        }
    }
    toggleDropdown() {
        this.isOpen = !this.isOpen;
        if (this.isOpen) {
            this.searchQuery = '';
            this.filteredCountries = this.countries;
        }
    }
    handleSearch(e) {
        const input = e.target;
        this.searchQuery = input.value.toLowerCase();
        this.filteredCountries = this.countries.filter(country => country.name.toLowerCase().includes(this.searchQuery) ||
            country.iocCode?.toLowerCase().includes(this.searchQuery) ||
            country.iso2Code?.toLowerCase().includes(this.searchQuery));
    }
    selectCountry(country) {
        this.selectedCountry = country;
        this.value = country.id;
        this.isOpen = false;
        // Dispatch change event
        this.dispatchEvent(new CustomEvent('change', {
            detail: { id: country.id, name: country.name },
            bubbles: true,
            composed: true
        }));
    }
    clearSelection() {
        this.selectedCountry = null;
        this.value = null;
        this.isOpen = false;
        // Dispatch change event
        this.dispatchEvent(new CustomEvent('change', {
            detail: { id: null, name: null },
            bubbles: true,
            composed: true
        }));
    }
    handleKeyDown(e) {
        if (e.key === 'Escape') {
            this.isOpen = false;
        }
    }
    firstUpdated() {
        document.addEventListener('click', this.handleClickOutside);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        document.removeEventListener('click', this.handleClickOutside);
    }
    render() {
        return html `
      <div class="selector-container">
        <button
          type="button"
          class="selected-display"
          @click=${this.toggleDropdown}
          @keydown=${this.handleKeyDown}
        >
          ${this.selectedCountry
            ? html `
                ${this.selectedCountry.iso2Code
                ? html `
                      <img
                        class="flag-icon"
                        src="/static/flags/${this.selectedCountry.iso2Code.toLowerCase()}.svg"
                        alt="${this.selectedCountry.name}"
                        @error=${(e) => {
                    e.target.style.display = 'none';
                }}
                      />
                    `
                : ''}
                <span>${this.selectedCountry.name}</span>
              `
            : html `<span class="placeholder">${this.placeholder}</span>`}
        </button>

        ${this.isOpen
            ? html `
              <div class="dropdown">
                <div class="search-box">
                  <input
                    type="text"
                    class="search-input"
                    placeholder="Search countries..."
                    .value=${this.searchQuery}
                    @input=${this.handleSearch}
                    @click=${(e) => e.stopPropagation()}
                  />
                </div>
                ${this.loading
                ? html `<div class="loading">Loading countries...</div>`
                : html `
                      <ul class="country-list">
                        ${!this.selectedCountry ? '' : html `
                          <li>
                            <button
                              type="button"
                              class="country-item"
                              @click=${this.clearSelection}
                            >
                              <span class="country-name">No country</span>
                            </button>
                          </li>
                        `}
                        ${this.filteredCountries.length === 0
                    ? html `<div class="no-results">No countries found</div>`
                    : this.filteredCountries.map(country => html `
                                <li>
                                  <button
                                    type="button"
                                    class="country-item ${this.selectedCountry?.id === country.id ? 'selected' : ''}"
                                    @click=${() => this.selectCountry(country)}
                                  >
                                    ${country.iso2Code
                        ? html `
                                          <img
                                            class="flag-icon"
                                            src="/static/flags/${country.iso2Code.toLowerCase()}.svg"
                                            alt="${country.name}"
                                            @error=${(e) => {
                            e.target.style.display = 'none';
                        }}
                                          />
                                        `
                        : ''}
                                    <span class="country-name">${country.name}</span>
                                    ${country.iocCode
                        ? html `<span class="country-code">${country.iocCode}</span>`
                        : ''}
                                  </button>
                                </li>
                              `)}
                      </ul>
                    `}
              </div>
            `
            : ''}

        <!-- Hidden input for form submission -->
        ${this.name
            ? html `
              <input
                type="hidden"
                name=${this.name}
                .value=${this.value?.toString() ?? ''}
              />
            `
            : ''}
      </div>
    `;
    }
};
CountrySelector.styles = css `
    :host {
      display: block;
      position: relative;
    }

    .selector-container {
      position: relative;
    }

    .selected-display {
      width: 100%;
      padding: 0.5rem;
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 4px;
      background: white;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      min-height: 38px;
    }

    .selected-display:hover {
      border-color: var(--gray-400, #9ca3af);
    }

    .selected-display:focus {
      outline: 2px solid var(--primary, #3b82f6);
      outline-offset: 2px;
    }

    .flag-icon {
      width: 20px;
      height: 15px;
      object-fit: cover;
      border: 1px solid var(--gray-300, #d1d5db);
    }

    .placeholder {
      color: var(--gray-500, #6b7280);
    }

    .dropdown {
      position: absolute;
      top: 100%;
      left: 0;
      right: 0;
      margin-top: 4px;
      background: white;
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 4px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
      max-height: 300px;
      overflow-y: auto;
      z-index: 1000;
    }

    .search-box {
      position: sticky;
      top: 0;
      background: white;
      padding: 0.5rem;
      border-bottom: 1px solid var(--gray-200, #e5e7eb);
    }

    .search-input {
      width: 100%;
      padding: 0.5rem;
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 4px;
      font-size: 0.875rem;
    }

    .search-input:focus {
      outline: 2px solid var(--primary, #3b82f6);
      outline-offset: -2px;
    }

    .country-list {
      list-style: none;
      margin: 0;
      padding: 0;
    }

    .country-item {
      padding: 0.5rem;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      border: none;
      background: none;
      width: 100%;
      text-align: left;
      font-size: 0.875rem;
    }

    .country-item:hover {
      background: var(--gray-100, #f3f4f6);
    }

    .country-item.selected {
      background: var(--primary-50, #eff6ff);
      color: var(--primary, #3b82f6);
    }

    .country-name {
      flex: 1;
    }

    .country-code {
      font-size: 0.75rem;
      color: var(--gray-500, #6b7280);
    }

    .no-results {
      padding: 1rem;
      text-align: center;
      color: var(--gray-500, #6b7280);
      font-size: 0.875rem;
    }

    .loading {
      padding: 1rem;
      text-align: center;
      color: var(--gray-500, #6b7280);
      font-size: 0.875rem;
    }
  `;
__decorate([
    property({ type: String })
], CountrySelector.prototype, "name", void 0);
__decorate([
    property({ type: Number })
], CountrySelector.prototype, "value", void 0);
__decorate([
    property({ type: String })
], CountrySelector.prototype, "placeholder", void 0);
__decorate([
    property({ type: Boolean, attribute: 'iihf-only' })
], CountrySelector.prototype, "iihfOnly", void 0);
__decorate([
    property({ type: Boolean, attribute: 'enabled-only' })
], CountrySelector.prototype, "enabledOnly", void 0);
__decorate([
    state()
], CountrySelector.prototype, "countries", void 0);
__decorate([
    state()
], CountrySelector.prototype, "filteredCountries", void 0);
__decorate([
    state()
], CountrySelector.prototype, "isOpen", void 0);
__decorate([
    state()
], CountrySelector.prototype, "searchQuery", void 0);
__decorate([
    state()
], CountrySelector.prototype, "loading", void 0);
__decorate([
    state()
], CountrySelector.prototype, "selectedCountry", void 0);
CountrySelector = __decorate([
    customElement('country-selector')
], CountrySelector);
export { CountrySelector };
//# sourceMappingURL=country-selector.js.map