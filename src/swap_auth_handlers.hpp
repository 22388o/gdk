#ifndef GDK_SWAP_AUTH_HANDLERS_HPP
#define GDK_SWAP_AUTH_HANDLERS_HPP
#pragma once

#include "auth_handler.hpp"

namespace green {

    class Tx;

    class create_swap_transaction_call : public auth_handler_impl {
    public:
        create_swap_transaction_call(session& session, const nlohmann::json& details);

    private:
        state_type call_impl() override;
        void on_next_handler_complete(auth_handler* next_handler) override;

        state_type liquidex_impl();

        nlohmann::json m_details;
        nlohmann::json m_receive_address;
        nlohmann::json m_create_details;
        bool m_is_signed;
    };

    class complete_swap_transaction_call : public auth_handler_impl {
    public:
        complete_swap_transaction_call(session& session, const nlohmann::json& details);

    private:
        state_type call_impl() override;
        void on_next_handler_complete(auth_handler* next_handler) override;

        state_type liquidex_impl();

        nlohmann::json m_details;
        nlohmann::json m_receive_address;
        nlohmann::json m_create_details;
        std::unique_ptr<Tx> m_tx;
    };

} // namespace green
#endif // GDK_SWAP_AUTH_HANDLERS_HPP
